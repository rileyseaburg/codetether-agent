//! Moltbook register response.
use super::*;

// ---------------------------------------------------------------------------
// Data types
// ---------------------------------------------------------------------------

/// Registration response from Moltbook.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Wire payload decoded from the Moltbook API.
///
/// # Examples
/// ```
/// let payload: codetether_moltbook::RegisterResponse = serde_json::from_str(
///     r#"{"agent":{"api_key":"example","claim_url":"https://example.invalid/claim","verification_code":"code"},"important":null}"#,
/// ).unwrap();
/// let _ = payload;
/// ```
pub struct RegisterResponse {
    /// API field `agent`.
    pub agent: RegisteredAgent,
    /// API field `important`.
    pub important: Option<String>,
    /// Whether the API key was persisted to HashiCorp Vault.
    #[serde(default)]
    /// API field `vault_saved`.
    pub vault_saved: bool,
}
