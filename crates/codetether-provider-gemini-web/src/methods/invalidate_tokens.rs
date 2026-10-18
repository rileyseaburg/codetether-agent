//! Provider invalidate tokens.
use crate::*;
impl GeminiWebProvider {
    /// Drop cached session tokens so the next request re-scrapes fresh values.
    #[cfg(test)]
    pub(crate) async fn invalidate_tokens(&self) {
        codetether_gemini_web_transport::GeminiWebSession::new(
            &self.client,
            &self.cookies,
            &self.token_cache,
        )
        .invalidate_tokens()
        .await
    }
}
