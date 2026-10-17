//! oauth client id.
use crate::*;
impl CodexOAuth {
    /// Returns the public OAuth client identifier used by the Codex flow.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_codex_auth::CodexOAuth;
    /// assert!(!CodexOAuth::oauth_client_id().is_empty());
    /// ```
    pub fn oauth_client_id() -> &'static str {
        CLIENT_ID
    }
}
