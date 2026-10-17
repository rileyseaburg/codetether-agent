//! Map non-success responses to the existing provider error text.
use crate::response::wire_error::*;
use crate::*;
pub(super) fn check(status: reqwest::StatusCode, text: &str) -> Result<()> {
    if !status.is_success() {
        if let Ok(err) = serde_json::from_str::<MoonshotError>(text) {
            anyhow::bail!(
                "Moonshot API error: {} ({:?})",
                err.error.message,
                err.error.error_type
            );
        }
        anyhow::bail!("Moonshot API error: {} {}", status, text);
    }

    Ok(())
}
