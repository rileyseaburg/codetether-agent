//! Provider ask.
use crate::*;
impl GeminiWebProvider {
    /// Core blocking request: POST and collect the complete response.
    pub(crate) async fn ask(&self, prompt: &str, model: &str) -> Result<String> {
        codetether_gemini_web_transport::GeminiWebSession::new(
            &self.client,
            &self.cookies,
            &self.token_cache,
        )
        .ask(prompt, model)
        .await
    }
}
