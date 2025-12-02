use crate::config::{CONFIG_FILE_NAME, Config};
use anyhow::{Result, bail};
use futures::future::try_join_all;
use serde_json::from_str;
use std::{path::Path, pin::Pin, sync::OnceLock};
use tokio::{
    fs::{self, File},
    io::{AsyncBufReadExt, BufReader},
    task::JoinHandle,
};
use tracing::{debug, error, info, warn};

type WalkResult = Result<Vec<JoinHandle<Result<()>>>>;

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

    let futs = check_recursive(path, &config.extensions).await?;

    let result = try_join_all(futs).await?;
    for r in result {
        if let Some(e) = r.err() {
            error!("{e}");
        }
    }

    info!("done");
    Ok(())
}

fn check_recursive<'r>(
    path: impl AsRef<Path> + 'r,
    extensions: &'r Vec<String>,
) -> Pin<Box<dyn Future<Output = WalkResult> + 'r>> {
    Box::pin(async move {
        let mut dir = match fs::read_dir(path).await {
            Ok(dir) => dir,
            Err(e) => {
                error!("{e}");
                return Ok(vec![]);
            }
        };

        let mut futs = vec![];

        while let Some(file) = dir.next_entry().await? {
            let path = file.path();
            let meta = file.metadata().await?;
            if meta.is_symlink() {
                continue;
            } else if meta.is_dir() {
                let mut rfuts = check_recursive(path, extensions).await?;
                futs.append(&mut rfuts);
            } else {
                let Some(ext_raw) = path.extension() else {
                    continue;
                };
                let Some(ext) = ext_raw.to_str() else {
                    continue;
                };
                if !extensions.contains(&ext.to_string()) {
                    continue;
                }

                let fut = tokio::spawn(check_one(path));
                futs.push(fut);
            }
        }

        Ok(futs)
    })
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
                warn!("found '{t}' at {} {l}:{}", path.as_ref().display(), idx + 1);
            }
        }
    }
    Ok(())
}
