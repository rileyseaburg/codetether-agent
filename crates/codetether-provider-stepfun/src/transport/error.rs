//! Typed API-error handling for buffered requests.
use crate::*;
pub(super) fn check(status: reqwest::StatusCode, body: &str) -> Result<()> {
    if !status.is_success() {
        if let Ok(err) = serde_json::from_str::<ErrorResponse>(body) {
            // Log error code if present for debugging
            if let Some(ref code) = err.error.code {
                tracing::error!(error_code = %code, "StepFun API error code");
            }
            anyhow::bail!("StepFun API error: {}", err.error.message);
        }
        anyhow::bail!("StepFun API error ({}): {}", status, body);
    }

    Ok(())
}
