//! Live validation for freshly minted Bedrock API keys.
//!
//! Uses the Bedrock management API with bearer auth before printing or
//! saving a token, so users know the exported key was accepted by AWS.
//! Validation targets the same region the token was signed for, because
//! short-term Bedrock API keys are region-scoped.

use super::token::OutputToken;
use anyhow::{Context, Result, bail};

/// Apply validation policy before any output can expose the token.
///
/// Validation must target the same region the token was signed for:
/// short-term Bedrock API keys are region-scoped and a key minted for one
/// region is rejected (403) by every other region's endpoint.
pub(super) async fn output_token(token: String, region: &str, skip: bool) -> Result<OutputToken> {
    if !skip {
        live_token(&token, region).await?;
        eprintln!("Validated Bedrock API key against AWS in region {region}.");
    }
    Ok(OutputToken::new(token))
}

/// Validate the bearer token against Bedrock's management API in `region`.
async fn live_token(token: &str, region: &str) -> Result<()> {
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
