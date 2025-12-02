use anyhow::Result;

async fn run() -> Result<()> {
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        panic!("{e}");
    }
}
