//! Streaming HTTP status admission and retry classification.
use crate::*;
pub(super) async fn admit(
    response: reqwest::Response,
    attempt: u32,
    last_err: &mut String,
) -> Result<Option<reqwest::Response>> {
    if response.status() == reqwest::StatusCode::SERVICE_UNAVAILABLE && attempt + 1 < MAX_RETRIES {
        let text = response.text().await.unwrap_or_default();
        tracing::warn!(body = %text, "Vertex GLM streaming service unavailable, retrying");
        *last_err = format!("503 Service Unavailable: {text}");
        return Ok(None);
    }

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        if let Ok(err) = serde_json::from_str::<ApiError>(&text) {
            anyhow::bail!(
                "Vertex AI GLM API error: {} ({:?})",
                err.error.message,
                err.error.error_type
            );
        }
        anyhow::bail!("Vertex AI GLM streaming error: {} {}", status, text);
    }
    Ok(Some(response))
}
