mod config;
mod telemetry;
mod signer;
mod solana_client;

use anyhow::Result;
use tokio::time::{sleep, Duration};
use telemetry::collect_metrics;
use signer::OperatorSigner;
use solana_client::submit_metrics;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let interval: u64 = std::env::var("SUBMIT_INTERVAL_SEC")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(60);

    let signer = OperatorSigner::from_env()?;
    loop {
        let m = collect_metrics().await?;
        submit_metrics(&signer, m).await?;
        sleep(Duration::from_secs(interval)).await;
    }
}
