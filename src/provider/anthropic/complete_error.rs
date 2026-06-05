//! Error mapping for Anthropic completion responses.

use super::error::AnthropicError;

/// Convert an unsuccessful HTTP response body into an anyhow error.
pub(crate) fn map(status: u16, text: &str) -> anyhow::Error {
    serde_json::from_str::<AnthropicError>(text).map_or_else(
        |_| anyhow::anyhow!("Anthropic API error: {status} {text}"),
        |err| {
            anyhow::anyhow!(
                "Anthropic API error: {} ({:?})",
                err.error.message,
                err.error.error_type
            )
        },
    )
}
