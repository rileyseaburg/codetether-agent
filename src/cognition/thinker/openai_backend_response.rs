//! Response decoding for the OpenAI-compatible thinker backend.

use anyhow::{Context, Result, anyhow};
use reqwest::Response;

use super::openai_wire::OpenAIChatResponse;
use super::{ThinkerConfig, ThinkerOutput};

/// Decode a successful chat-completions response into a [`ThinkerOutput`].
pub(super) async fn decode(config: &ThinkerConfig, response: Response) -> Result<ThinkerOutput> {
    let payload: OpenAIChatResponse = response
        .json()
        .await
        .context("failed to decode thinker response")?;
    let choice = payload
        .choices
        .first()
        .ok_or_else(|| anyhow!("thinker response did not include choices"))?;
    let text = choice.message.extract_text();
    let usage = payload.usage.unwrap_or_default();

    Ok(ThinkerOutput {
        model: payload.model.unwrap_or_else(|| config.model.clone()),
        finish_reason: choice.finish_reason.clone(),
        text,
        prompt_tokens: usage.prompt_tokens,
        completion_tokens: usage.completion_tokens,
        total_tokens: usage.total_tokens,
        cache_read_tokens: None,
        cache_write_tokens: None,
    })
}

/// Classify a non-retryable, unsuccessful response into an error.
pub(super) async fn status_error(response: Response) -> anyhow::Error {
    let status = response.status();
    let body_text = response
        .text()
        .await
        .unwrap_or_else(|_| "<empty>".to_string());
    anyhow!("thinker request failed with status {status}: {body_text}")
}
