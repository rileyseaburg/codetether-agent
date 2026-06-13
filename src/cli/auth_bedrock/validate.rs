//! Live validation for freshly minted Bedrock API keys.
//!
//! Uses the Bedrock management API with bearer auth before printing or
//! saving a token, so users know the exported key was accepted by AWS.

use anyhow::{Context, Result, bail};

/// Validate the bearer token against Bedrock's management API.
pub(super) async fn token(region: &str, token: &str) -> Result<()> {
    let url = format!("https://bedrock.{region}.amazonaws.com/foundation-models");
    let resp = crate::provider::shared_http::shared_client()
        .get(&url)
        .bearer_auth(token)
        .header("accept", "application/json")
        .send()
        .await
        .context("Failed to call Bedrock validation endpoint")?;
    if resp.status().is_success() {
        return Ok(());
    }
    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();
    bail!(
        "Minted Bedrock API key was rejected by AWS ({status}). Response: {}",
        body.trim()
    )
}
