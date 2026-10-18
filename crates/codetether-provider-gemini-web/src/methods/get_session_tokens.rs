//! Provider get session tokens.
use crate::*;
impl GeminiWebProvider {
    /// GET the Gemini home page and extract the three tokens we need:
    ///   - `at_token`  XSRF token (key `thykhd` or legacy `SNlM0e`)
    ///   - `f_sid`     session ID (`FdrFJe`)
    ///   - `bl`        build label (`cfb2h`)
    ///
    /// Regexes are compiled once and reused across calls via `OnceLock`.
    #[cfg(test)]
    pub(crate) async fn get_session_tokens(&self) -> Result<SessionTokens> {
        codetether_gemini_web_transport::GeminiWebSession::new(
            &self.client,
            &self.cookies,
            &self.token_cache,
        )
        .get_session_tokens()
        .await
    }
}
