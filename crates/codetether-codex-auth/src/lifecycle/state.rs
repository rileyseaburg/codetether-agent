//! Shared OAuth state and an unpersisted rotation marker.
use super::*;
/// Current credentials and a pending compare-before-write marker.
///
/// # Examples
/// ```
/// # let credentials = codetether_codex_auth::OAuthCredentials { access_token: "access".into(), refresh_token: "refresh".into(), expires_at: 0, id_token: None, chatgpt_account_id: None };
/// let state = codetether_codex_auth::lifecycle::OAuthCredentialState::new(credentials);
/// assert!(state.pending_refresh_token.is_none());
/// ```
pub struct OAuthCredentialState {
    /// Most recent local credentials.
    pub credentials: OAuthCredentials,
    /// Prior refresh token expected by an unpersisted rotation.
    pub pending_refresh_token: Option<String>,
}

impl OAuthCredentialState {
    /// Initialize local state with no pending persistence.
    pub fn new(credentials: OAuthCredentials) -> Self {
        Self {
            credentials,
            pending_refresh_token: None,
        }
    }
}
