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
use tracing::{Level, error};

async fn run(args: Args) -> Result<()> {
    match args.cmd {
        args::Command::Config => run_config().await,
        args::Command::Check { path } => run_check(&path).await,
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let args = Args::parse();

    if let Err(e) = run(args).await {
        if cfg!(debug_assertions) {
            panic!("{e}");
        } else {
            error!("{e}");
        }
    }
}
