//! Signed Bedrock dispatch and success logging for the shared thinker service.
use super::{ThinkerConfig, ThinkerOutput};
use crate::provider::bedrock::BedrockProvider;
use anyhow::Result;
use std::time::Instant;

/// Generate a thought via the Bedrock Converse API.
pub(super) async fn think(
    config: &ThinkerConfig,
    provider: &BedrockProvider,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<ThinkerOutput> {
    codetether_cognition_core::thinker::bedrock::think(
        config,
        system_prompt,
        user_prompt,
        |url, bytes| async move { provider.send_converse_request(&url, &bytes).await },
        |output, started_at| log_success(config, output, started_at),
    )
    .await
}

fn log_success(config: &ThinkerConfig, output: &ThinkerOutput, started_at: Instant) {
    tracing::debug!(
        model = %config.model,
        latency_ms = started_at.elapsed().as_millis(),
        prompt_tokens = ?output.prompt_tokens,
        completion_tokens = ?output.completion_tokens,
        "bedrock thinker generated thought"
    );
}
