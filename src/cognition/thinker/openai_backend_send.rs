//! Single-attempt HTTP send for the OpenAI-compatible thinker backend.

use anyhow::Result;
use reqwest::{Client, Response};

use super::ThinkerConfig;
use super::openai_wire::OpenAIChatRequest;
use super::retry::is_transient_reqwest_error;

/// Outcome of one send attempt: `Err` inside `Ok` marks a retryable failure.
pub(super) type Attempt = std::result::Result<Response, anyhow::Error>;

/// Send one chat-completions request.
///
/// # Errors
///
/// Returns `Err` only for non-retryable transport failures.
pub(super) async fn send(
    config: &ThinkerConfig,
    http: &Client,
    body: &OpenAIChatRequest,
) -> Result<Attempt> {
    let mut request = http.post(&config.endpoint).json(body);
    if let Some(key) = config.api_key.as_ref() {
        request = request.bearer_auth(key);
    }
    match request.send().await {
        Ok(response) => Ok(Ok(response)),
        Err(e) if is_transient_reqwest_error(&e) => {
            tracing::warn!(error = %e, "thinker HTTP request failed (transient)");
            Ok(Err(
                anyhow::Error::from(e).context("transient thinker send error")
            ))
        }
        Err(e) => Err(anyhow::Error::from(e).context("non-transient thinker send error")),
    }
}
