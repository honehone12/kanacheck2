mod args;
mod commands {
    pub mod check;
    pub mod config;
}
mod config;

use crate::args::Args;
use crate::commands::{check::run_check, config::run_config};
use anyhow::Result;
use clap::Parser;
use tracing::Level;

async fn run(args: Args) -> Result<()> {
    match args.cmd {
        args::Command::Config => run_config().await,
        args::Command::Check { path } => run_check(&path).await,
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    let args = Args::parse();

    if let Err(e) = run(args).await {
        panic!("{e}");
    }
}
