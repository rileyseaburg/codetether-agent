//! Buffered response text and HTTP errors.
use crate::*;
pub(super) async fn read(response: reqwest::Response) -> Result<String> {
    let status = response.status();
    let text = response
        .text()
        .await
        .context("Failed to read GLM-5 FP8 response")?;

    if !status.is_success() {
        if let Ok(err) = serde_json::from_str::<Glm5Error>(&text) {
            anyhow::bail!(
                "GLM-5 FP8 API error: {} ({:?})",
                err.error.message,
                err.error.error_type
            );
        }
        anyhow::bail!("GLM-5 FP8 API error: {} {}", status, text);
    }
    Ok(text)
}
