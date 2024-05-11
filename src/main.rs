mod cli;
mod prop;
mod types;

use std::fmt::format;
use std::io::BufRead;
use std::time::Duration;
use clap::Parser;
use log::{error, info};
use nix::sys::stat::Mode;
use serde::{Deserialize, Serialize};



fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let args = cli::Args::parse();
    println!("args: {:?}", args);

    if std::fs::remove_file(&args.pipe_path).is_ok() {
        info!("original file at {:?} removed", args.pipe_path);
    }

    let fifo_mode = Mode::S_IRWXU.union(Mode::S_IRWXG).union(Mode::S_IRWXO);
    nix::unistd::mkfifo(&args.pipe_path, fifo_mode)?;
    let pipe = std::fs::File::open(&args.pipe_path)?;
    let mut pipe = std::io::BufReader::new(pipe);

    let mut propagator = prop::Propagator::new(&args);

    let mut line = String::new();
    loop {
        while let Ok(count) = pipe.read_line(&mut line) {
            if count < 1 { break; }
            let Ok(msg) = serde_json::from_str::<types::Log>(&line) else {
                error!("malformed json");
                break;
            };
            propagator.add(msg);
            line.clear();
        }
        line.clear();
        propagator.propagate();
        std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
