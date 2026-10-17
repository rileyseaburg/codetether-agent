//! Status gate for unconsumed HTTP responses.
use super::*;

/// Reject HTTP errors with retry metadata, or start the response stream.
///
/// # Arguments
/// * `response` - An unconsumed HTTP response.
/// * `model` - Requested model for authentication diagnostics.
/// # Returns
/// A decoded chunk stream for successful HTTP responses.
/// # Errors
/// Returns the original status/body diagnostic and optional retry delay marker.
pub async fn validate_http_response(
    response: reqwest::Response,
    model: &str,
) -> Result<crate::Chunks> {
    let status = response.status();
    if !status.is_success() {
        let retry_after = retry_after_seconds(response.headers());
        let body = response.text().await.unwrap_or_default();
        let message = super::format_openai_api_error(status, &body, model);
        anyhow::bail!(annotate_retry_after(message, retry_after));
    }
    Ok(crate::drive_responses_http_stream(response))
}
