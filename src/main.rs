use std::io::BufRead;
use std::sync::mpsc;
use std::time::Duration;

use clap::Parser;
use log::{error, info};

mod cli;
mod prop;
mod types;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let args = cli::Args::parse();
    println!("args: {:?}", args);

    if std::fs::remove_file(&args.pipe_path).is_ok() {
        info!("original file at {:?} removed", args.pipe_path);
    }

    let pipe = std::fs::File::open(&args.pipe_path)?;
    let mut pipe = std::io::BufReader::new(pipe);

    let (tx, rx) = mpsc::channel::<types::Log>();
    let child = std::thread::spawn(move || {
        let rx = rx;
        let args = args.clone();
        let mut propagator = prop::Propagator::new(&args);
        loop {
            while let Ok(msg) = rx.recv_timeout(Duration::from_millis(500)) {
                propagator.add(msg);
            }
            propagator.propagate();
        }
    });

    let mut line = String::new();
    loop {
        while let Ok(count) = pipe.read_line(&mut line) {
            if count < 1 {
                break;
            }
            let line_trimmed = line.trim();
            if line_trimmed.is_empty() {
                break;
            }
            let Ok(msg) = serde_json::from_str::<types::Log>(line_trimmed) else {
                error!("malformed json");
                break;
            };
            println!("{:?}", msg);
            tx.send(msg).unwrap();
            line.clear();
        }
        line.clear();
        std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}
