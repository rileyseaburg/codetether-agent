//! Buffered HTTP error decoding.
use crate::*;
pub(super) fn check(status: reqwest::StatusCode, text: &str) -> Result<()> {
    if !status.is_success() {
        if let Ok(err) = serde_json::from_str::<AnthropicError>(text) {
            anyhow::bail!(
                "Vertex AI Anthropic API error: {} ({:?})",
                err.error.message,
                err.error.error_type
            );
        }
        anyhow::bail!("Vertex AI Anthropic API error: {} {}", status, text);
    }

    Ok(())
}
