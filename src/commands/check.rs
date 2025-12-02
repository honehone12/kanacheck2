use crate::config::{CONFIG_FILE_NAME, Config};
use anyhow::{Result, bail};
use futures::future::try_join_all;
use serde_json::from_str;
use std::{path::Path, sync::OnceLock};
use tokio::{
    fs::{self, File},
    io::{AsyncBufReadExt, BufReader},
};
use tracing::{debug, error, info, warn};

static TARGETS: OnceLock<Vec<String>> = OnceLock::new();

pub async fn run_check(path: &str) -> Result<()> {
    let path = Path::new(path);
    debug!("path: {path:?}");

    if !fs::try_exists(CONFIG_FILE_NAME).await? {
        bail!("please generate config file with config command");
    }

    let s = fs::read_to_string(CONFIG_FILE_NAME).await?;
    let config = from_str::<Config>(&s)?;
    if let Err(_) = TARGETS.set(config.characters) {
        bail!("targets cell is already initialized");
    }

    let meta = fs::metadata(path).await?;
    if !meta.is_dir() {
        bail!("{path:?} is not directory, please specify directory");
    }

    let mut dir = fs::read_dir(path).await?;
    let mut futs = vec![];

    while let Some(file) = dir.next_entry().await? {
        let meta = file.metadata().await?;
        if !meta.is_file() {
            continue;
        }

        let path = file.path();

        let Some(ext_raw) = path.extension() else {
            continue;
        };
        let Some(ext) = ext_raw.to_str() else {
            continue;
        };
        if !config.extensions.contains(&ext.to_string()) {
            continue;
        }

        let fut = tokio::spawn(check_one(path));
        futs.push(fut);
    }

    let result = try_join_all(futs).await?;
    for r in result {
        if let Some(e) = r.err() {
            error!("{e}");
        }
    }

    info!("done");
    Ok(())
}

async fn check_one(path: impl AsRef<Path>) -> Result<()> {
    let file = File::open(&path).await?;
    let mut stream = BufReader::new(file).lines();
    let Some(targets) = TARGETS.get() else {
        bail!("targets cell is not initialized");
    };

    let mut l = 0u64;
    while let Some(line) = stream.next_line().await? {
        l += 1;

        if line.is_empty() {
            continue;
        }

        for t in targets {
            if let Some(idx) = line.find(t) {
                warn!("found '{t}' at　{} {l}:{idx}", path.as_ref().display());
            }
        }
    }
    Ok(())
}
