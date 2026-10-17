//! Translate an HTTP failure into the existing provider error.
use super::error::DsError;
pub(super) fn map_error(code: u16, body: &str) -> anyhow::Error {
    if let Ok(err) = serde_json::from_str::<DsError>(body) {
        anyhow::anyhow!(
            "DeepSeek API error: {} ({:?})",
            err.error.message,
            err.error.error_type
        )
    } else {
        anyhow::anyhow!("DeepSeek API error: {code} {body}")
    }
}
