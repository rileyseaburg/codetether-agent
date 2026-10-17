//! InvokeModel HTTP error mapping (retention denials, generic errors).
//!
//! Kept separate from the retry loop in `complete.rs` so each file owns a
//! single concern (SRP) and stays within the line budget.

use codetether_bedrock_protocol::BedrockError;
use reqwest::StatusCode;

/// Map a non-success InvokeModel HTTP response to an [`anyhow::Error`],
/// surfacing data-retention recovery guidance when applicable.
pub fn invoke_error(status: StatusCode, text: &str, model_id: &str, region: &str) -> anyhow::Error {
    tracing::debug!(
        provider = "bedrock",
        model = %model_id,
        status = %status,
        raw = %codetether_util::truncate_bytes_safe(text, 800),
        "Bedrock InvokeModel error response"
    );
    let detail = match serde_json::from_str::<BedrockError>(text) {
        Ok(err) => err.message,
        Err(_) => codetether_util::truncate_bytes_safe(text, 500).to_string(),
    };
    let base = format!("Bedrock InvokeModel error ({status}): {detail}");
    if super::retention::is_retention_denied(&detail) {
        return anyhow::anyhow!(super::retention::guidance(&base, model_id, region));
    }
    anyhow::anyhow!(codetether_bedrock_protocol::audit::pairing_error::annotate(
        &base, &detail
    ))
}
