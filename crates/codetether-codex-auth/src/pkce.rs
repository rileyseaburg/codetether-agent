//! PKCE verifier and challenge returned by the current generator.

/// PKCE code verifier and challenge pair
///
/// # Examples
/// ```
/// let pair = codetether_codex_auth::CodexOAuth::generate_pkce();
/// assert!(!pair.verifier.is_empty() && !pair.challenge.is_empty());
/// ```
pub struct PkcePair {
    /// Verifier sent during code exchange.
    pub verifier: String,
    /// S256 challenge used in the authorization URL.
    pub challenge: String,
}
