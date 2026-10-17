//! Streaming HTTP error handling.
use crate::*;
pub(super) async fn admit(response: reqwest::Response) -> Result<reqwest::Response> {
    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        if let Ok(err) = serde_json::from_str::<Glm5Error>(&text) {
            anyhow::bail!(
                "GLM-5 FP8 API error: {} ({:?})",
                err.error.message,
                err.error.error_type
            );
        }
        anyhow::bail!("GLM-5 FP8 streaming error: {} {}", status, text);
    }
    Ok(response)
}
