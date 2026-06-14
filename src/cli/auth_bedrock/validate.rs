//! Live validation for freshly minted Bedrock API keys.
//!
//! Uses the Bedrock management API with bearer auth before printing or
//! saving a token, so users know the exported key was accepted by AWS.
//! Validation always targets `us-east-1` regardless of the signing region.

use super::token::OutputToken;
use crate::provider::bedrock::DEFAULT_REGION;
use anyhow::{Context, Result, bail};

/// Apply validation policy before any output can expose the token.
pub(super) async fn output_token(token: String, skip: bool) -> Result<OutputToken> {
    if !skip {
        live_token(&token).await?;
        eprintln!("Validated Bedrock API key against AWS in region {DEFAULT_REGION}.");
    }
    Ok(OutputToken::new(token))
}

/// Validate the bearer token against Bedrock's management API in `us-east-1`.
async fn live_token(token: &str) -> Result<()> {
    let url = format!("https://bedrock.{DEFAULT_REGION}.amazonaws.com/foundation-models");
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
