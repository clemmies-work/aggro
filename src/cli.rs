use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub host: String,
    #[arg(long)]
    pub port: u16,
    #[arg(long)]
    pub username: String,
    #[arg(long)]
    pub password: String,
    #[arg(value_name = "PIPE_PATH")]
    pub pipe_path: PathBuf,
}
