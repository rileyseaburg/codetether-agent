//! Provider-owned client, cookie string, and shared token cache.
use crate::*;

/// Gemini Web provider with cookie authentication and shared token caching.
///
/// # Examples
/// ```no_run
/// let provider = codetether_provider_gemini_web::GeminiWebProvider::new(String::new()).unwrap();
/// assert!(format!("{provider:?}").contains("[redacted]"));
/// ```
pub struct GeminiWebProvider {
    pub(crate) client: Client,
    /// Raw contents of cookies.txt (Netscape / Cookie-Editor export format)
    pub(crate) cookies: String,
    /// Cached session tokens with the instant they were fetched.
    pub(crate) token_cache: Mutex<Option<(SessionTokens, Instant)>>,
}

impl std::fmt::Debug for GeminiWebProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GeminiWebProvider")
            .field("cookies", &"[redacted]")
            .finish_non_exhaustive()
    }
}
