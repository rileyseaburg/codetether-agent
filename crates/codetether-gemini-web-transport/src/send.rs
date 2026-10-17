//! Send one prepared Gemini request with the original error context.
use crate::*;
impl GeminiWebSession<'_> {
    pub(crate) async fn send_request(
        &self,
        prompt: &str,
        model: &str,
    ) -> Result<reqwest::Response> {
        self.build_request(prompt, model)
            .await?
            .send()
            .await
            .context("Failed to send request to Gemini StreamGenerate")
    }
}
