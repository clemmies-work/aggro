mod cli;

use std::io::BufRead;
use std::time::Duration;
use clap::Parser;
use log::{error, info};
use nix::sys::stat::Mode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Log {
    us_since_unix_epoch: u128,
    process_id: u32,

    file_path: String,
    function_name: String,
    line_of_code: usize,

    content: String,
}

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
    let mut line = String::new();
    while let Ok(count) = pipe.read_line(&mut line) {
        std::thread::sleep(Duration::from_millis(1500));
        if count < 1 { continue; }
        match serde_json::from_str::<Log>(&line) {
            Ok(msg) => {
                error!("{:?}", msg);
            },
            Err(e) => {
                error!("received malformed data: {:?}", e);
            },
        };
        line.clear();
    }

    Ok(())
}
