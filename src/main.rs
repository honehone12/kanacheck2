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

async fn run(args: Args) -> Result<()> {
    match args.cmd {
        args::Command::Config => run_config().await,
        args::Command::Check {} => run_check().await,
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if let Err(e) = run(args).await {
        panic!("{e}");
    }
}
