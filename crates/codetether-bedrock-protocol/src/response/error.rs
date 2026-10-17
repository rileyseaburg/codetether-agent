//! Service error payload.
use serde::Deserialize;

/// Error body returned by Bedrock when a request is rejected.
///
/// # Examples
/// ```
/// let error: codetether_bedrock_protocol::BedrockError = serde_json::from_str(r#"{"message":"rejected"}"#)?;
/// assert_eq!(error.message, "rejected");
/// # Ok::<(), serde_json::Error>(())
/// ```
#[derive(Debug, Deserialize)]
pub struct BedrockError {
    /// Human-readable error message.
    pub message: String,
}
