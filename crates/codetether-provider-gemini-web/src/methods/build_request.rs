//! Provider build request.
use crate::*;
impl GeminiWebProvider {
    /// Build a `RequestBuilder` for the StreamGenerate endpoint.
    pub(crate) async fn build_request(
        &self,
        prompt: &str,
        model: &str,
    ) -> Result<reqwest::RequestBuilder> {
        codetether_gemini_web_transport::GeminiWebSession::new(
            &self.client,
            &self.cookies,
            &self.token_cache,
        )
        .build_request(prompt, model)
        .await
    }
}
