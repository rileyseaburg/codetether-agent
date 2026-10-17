//! Moltbook claim status.
use super::*;

/// Claim status.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Wire payload decoded from the Moltbook API.
///
/// # Examples
/// ```
/// let payload: codetether_moltbook::ClaimStatus = serde_json::from_str(
///     r#"{"status":"pending"}"#,
/// ).unwrap();
/// let _ = payload;
/// ```
pub struct ClaimStatus {
    /// API field `status`.
    pub status: String,
}
