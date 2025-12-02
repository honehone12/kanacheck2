use crate::config::{CONFIG_FILE_NAME, Config};
use anyhow::Result;
use serde_json::to_string_pretty;
use tokio::fs;

pub async fn run_config() -> Result<()> {
    let conf = Config::default();
    let s = to_string_pretty(&conf)?;
    fs::write(CONFIG_FILE_NAME, s.as_bytes()).await?;
    Ok(())
}
