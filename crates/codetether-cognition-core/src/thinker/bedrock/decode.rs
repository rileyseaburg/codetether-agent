//! Consume and convert a Bedrock HTTP response.
use super::{ThinkerConfig, ThinkerOutput};
use crate::thinker::protocol::bedrock_response;
use anyhow::{Context, Result, anyhow};

pub(super) async fn decode(
    config: &ThinkerConfig,
    response: reqwest::Response,
) -> Result<ThinkerOutput> {
    let status = response.status();
    let text = response
        .text()
        .await
        .context("Failed to read Bedrock thinker response")?;

    if !status.is_success() {
        return Err(anyhow!(
            "Bedrock thinker error ({}): {}",
            status,
            codetether_util::truncate::truncate_bytes_safe(&text, 500)
        ));
    }

    let parsed: serde_json::Value =
        serde_json::from_str(&text).context("Failed to parse Bedrock thinker response")?;
    let output = bedrock_response::decode(&config.model, &parsed);

    Ok(output)
}
