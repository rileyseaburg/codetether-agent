//! Response decoding for the OpenAI-compatible thinker backend.

use anyhow::{Context, Result, anyhow};
use reqwest::Response;

use super::{ThinkerConfig, ThinkerOutput};
use crate::thinker::protocol::openai_wire::OpenAIChatResponse;

/// Decode a successful chat-completions response into a [`ThinkerOutput`].
pub(super) async fn decode(config: &ThinkerConfig, response: Response) -> Result<ThinkerOutput> {
    let payload: OpenAIChatResponse = response
        .json()
        .await
        .context("failed to decode thinker response")?;
    crate::thinker::protocol::openai_response::from_payload(config, payload)
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
