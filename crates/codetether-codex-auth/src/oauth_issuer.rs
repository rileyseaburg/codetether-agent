//! oauth issuer.
use crate::*;
impl CodexOAuth {
    /// Returns the trusted OAuth issuer URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_codex_auth::CodexOAuth;
    /// assert_eq!(CodexOAuth::oauth_issuer(), "https://auth.openai.com");
    /// ```
    pub fn oauth_issuer() -> &'static str {
        AUTH_ISSUER
    }
}
