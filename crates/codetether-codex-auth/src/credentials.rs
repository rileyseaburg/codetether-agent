//! Serializable OAuth credentials shared by exchange and provider storage.
use serde::{Deserialize, Serialize};

/// OAuth credentials persisted by the provider's secret store.
///
/// # Examples
///
/// ```
/// use codetether_codex_auth::OAuthCredentials;
///
/// let credentials = OAuthCredentials {
///     id_token: None,
///     chatgpt_account_id: Some("workspace-1".to_string()),
///     access_token: "access".to_string(),
///     refresh_token: "refresh".to_string(),
///     expires_at: 1_900_000_000,
/// };
/// assert_eq!(credentials.chatgpt_account_id.as_deref(), Some("workspace-1"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthCredentials {
    /// Optional OpenID Connect identity token returned by OAuth.
    #[serde(default)]
    pub id_token: Option<String>,
    /// ChatGPT workspace identifier associated with the credentials.
    #[serde(default)]
    pub chatgpt_account_id: Option<String>,
    /// Bearer token used for authenticated requests.
    pub access_token: String,
    /// Token used to renew an expired access token.
    pub refresh_token: String,
    /// Access-token expiration as Unix seconds.
    pub expires_at: u64,
}
