//! Session credentials scraped from the selected Gemini account.

/// XSRF, routing, and build tokens extracted from one Gemini page.
///
/// # Examples
/// ```
/// let tokens = codetether_gemini_web_transport::SessionTokens {
///     at_token: "token".into(), f_sid: "session".into(),
///     bl: "build".into(), acct_prefix: String::new(),
/// };
/// assert!(tokens.acct_prefix.is_empty());
/// ```
#[derive(Clone)]
pub struct SessionTokens {
    /// Current XSRF token.
    pub at_token: String,
    /// Session routing identifier.
    pub f_sid: String,
    /// Backend build label.
    pub bl: String,
    /// Path prefix extracted from the /u/N/ portion of the home-page redirect
    /// URL (e.g. "/u/1"). Empty for the primary account which uses no prefix.
    pub acct_prefix: String,
}
