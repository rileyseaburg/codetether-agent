//! Gemini Web get or refresh tokens.
use crate::*;
impl GeminiWebSession<'_> {
    /// Return session tokens, re-fetching only when the cached copy has
    /// exceeded `TOKEN_TTL` (20 minutes).
    ///
    /// # Errors
    /// Returns page transport, body-read, or required-token extraction errors.
    pub async fn get_or_refresh_tokens(&self) -> Result<SessionTokens> {
        let mut cache = self.token_cache.lock().await;
        if let Some((ref tokens, fetched_at)) = *cache
            && fetched_at.elapsed() < TOKEN_TTL
        {
            return Ok(tokens.clone());
        }
        let fresh = self.get_session_tokens().await?;
        *cache = Some((fresh.clone(), Instant::now()));
        Ok(fresh)
    }
}
