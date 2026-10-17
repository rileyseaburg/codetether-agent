//! Buffered HTTP status handling and response text.
use crate::*;
pub(super) async fn read(
    response: reqwest::Response,
    attempt: u32,
    last_err: &mut Option<String>,
) -> Result<Option<String>> {
    let status = response.status();
    let text = response
        .text()
        .await
        .context("Failed to read Vertex AI GLM response")?;

    if status == reqwest::StatusCode::SERVICE_UNAVAILABLE && attempt + 1 < MAX_RETRIES {
        tracing::warn!(status = %status, body = %text, "Vertex GLM service unavailable, retrying");
        *last_err = Some(format!("503 Service Unavailable: {text}"));
        return Ok(None);
    }

    if !status.is_success() {
        if let Ok(err) = serde_json::from_str::<ApiError>(&text) {
            anyhow::bail!(
                "Vertex AI GLM API error: {} ({:?})",
                err.error.message,
                err.error.error_type
            );
        }
        anyhow::bail!("Vertex AI GLM API error: {} {}", status, text);
    }
    Ok(Some(text))
}
