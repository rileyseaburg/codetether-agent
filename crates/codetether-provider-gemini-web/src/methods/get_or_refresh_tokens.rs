//! Provider get or refresh tokens.
use crate::*;
impl GeminiWebProvider {
    /// Return session tokens, re-fetching only when the cached copy has
    /// exceeded `TOKEN_TTL` (20 minutes).
    pub(crate) async fn get_or_refresh_tokens(&self) -> Result<SessionTokens> {
        codetether_gemini_web_transport::GeminiWebSession::new(
            &self.client,
            &self.cookies,
            &self.token_cache,
        )
        .get_or_refresh_tokens()
        .await
    }
}
