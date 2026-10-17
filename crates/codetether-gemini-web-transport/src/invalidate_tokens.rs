//! Gemini Web invalidate tokens.
use crate::*;
impl GeminiWebSession<'_> {
    /// Drop cached session tokens so the next request re-scrapes fresh values.
    pub async fn invalidate_tokens(&self) {
        let mut cache = self.token_cache.lock().await;
        *cache = None;
    }
}
