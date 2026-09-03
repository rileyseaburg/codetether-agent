//! Non-2xx Converse response → actionable error.
//!
//! Split out of [`super`] so the retry loop stays focused on dispatch while
//! this file owns body parsing and pairing-failure annotation.

use crate::provider::bedrock::BedrockError;
use crate::provider::bedrock::body::audit::pairing_error;
use crate::util;
use reqwest::StatusCode;

/// Map a failed Converse response to an error carrying the service message.
pub(super) fn map(status: StatusCode, text: &str) -> anyhow::Error {
    if let Ok(err) = serde_json::from_str::<BedrockError>(text) {
        let base = format!("Bedrock API error ({status}): {}", err.message);
        return anyhow::anyhow!(pairing_error::annotate(&base, text));
    }
    anyhow::anyhow!(
        "Bedrock API error: {status} {}",
        util::truncate_bytes_safe(text, 500)
    )
}
