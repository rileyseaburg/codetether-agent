//! Send a prepared release request and enforce HTTP success.
use anyhow::{Context, Result};

pub async fn send_release(request: reqwest::RequestBuilder) -> Result<()> {
    let response = request
        .send()
        .await
        .context("Failed to send task release request")?;
    let status = response.status();
    if status.is_success() {
        return Ok(());
    }
    let body = response.text().await.unwrap_or_default();
    anyhow::bail!("Task release failed with HTTP {status}: {body}")
}
