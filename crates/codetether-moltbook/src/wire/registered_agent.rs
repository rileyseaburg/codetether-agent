//! Moltbook registered agent.
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Wire payload decoded from the Moltbook API.
///
/// # Examples
/// ```
/// let payload: codetether_moltbook::RegisteredAgent = serde_json::from_str(
///     r#"{"api_key":"example","claim_url":"https://example.invalid/claim","verification_code":"code"}"#,
/// ).unwrap();
/// let _ = payload;
/// ```
pub struct RegisteredAgent {
    /// API field `api_key`.
    pub api_key: String,
    /// API field `claim_url`.
    pub claim_url: String,
    /// API field `verification_code`.
    pub verification_code: String,
}
