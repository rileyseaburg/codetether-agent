//! Bedrock Converse thinker backend.

use anyhow::{Context, Result, anyhow};
use std::time::Instant;

use super::{ThinkerConfig, ThinkerOutput, bedrock_request, bedrock_response};
use crate::provider::bedrock::BedrockProvider;

/// Generate a thought via the Bedrock Converse API.
pub(super) async fn think(
    config: &ThinkerConfig,
    provider: &BedrockProvider,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<ThinkerOutput> {
    let started_at = Instant::now();
    let body = bedrock_request::body(config, system_prompt, user_prompt);
    let body_bytes = serde_json::to_vec(&body)?;

    let response = provider
        .send_converse_request(&bedrock_request::url(config), &body_bytes)
        .await
        .context("Bedrock thinker converse request failed")?;

    let status = response.status();
    let text = response
        .text()
        .await
        .context("Failed to read Bedrock thinker response")?;

    if !status.is_success() {
        return Err(anyhow!(
            "Bedrock thinker error ({}): {}",
            status,
            crate::util::truncate_bytes_safe(&text, 500)
        ));
    }

    let parsed: serde_json::Value =
        serde_json::from_str(&text).context("Failed to parse Bedrock thinker response")?;
    let output = bedrock_response::decode(&config.model, &parsed);

    tracing::debug!(
        model = %config.model,
        latency_ms = started_at.elapsed().as_millis(),
        prompt_tokens = ?output.prompt_tokens,
        completion_tokens = ?output.completion_tokens,
        "bedrock thinker generated thought"
    );
    Ok(output)
}
