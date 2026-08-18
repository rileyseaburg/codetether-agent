//! Retryable/usable classification of OpenAI-compatible HTTP responses.

use anyhow::{Result, anyhow};
use reqwest::Response;

use super::openai_backend_response;
use super::retry::is_transient_http_error;

/// A classified response: `Err` inside `Ok` marks a retryable status.
pub(super) type Classified = std::result::Result<Response, anyhow::Error>;

/// Split a response into retryable (`Err`) and usable (`Ok`) outcomes.
///
/// # Errors
///
/// Returns `Err` for unsuccessful statuses that must not be retried.
pub(super) async fn classify(response: Response) -> Result<Classified> {
    let status = response.status();
    if is_transient_http_error(status.as_u16()) {
        let body_text = response.text().await.unwrap_or_default();
        tracing::warn!(%status, "thinker received transient HTTP error");
        return Ok(Err(anyhow!(
            "thinker request failed with status {status}: {body_text}"
        )));
    }
    if !status.is_success() {
        return Err(openai_backend_response::status_error(response).await);
    }
    Ok(Ok(response))
}
