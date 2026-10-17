//! Non-success HTTP responses.
use crate::response::wire_error::*;
use crate::*;
pub(super) fn check(status: reqwest::StatusCode, text: &str) -> Result<()> {
    if !status.is_success() {
        if let Ok(err) = serde_json::from_str::<ApiError>(text) {
            anyhow::bail!("Google Gemini API error: {}", err.error.message);
        }
        anyhow::bail!("Google Gemini API error: {} {}", status, text);
    }

    Ok(())
}
