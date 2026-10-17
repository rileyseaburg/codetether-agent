//! Buffered request orchestration.
use crate::*;
impl VertexAnthropicProvider {
    pub(crate) async fn complete_request(
        &self,
        request: CompletionRequest,
    ) -> Result<CompletionResponse> {
        let access_token = self.get_access_token().await?;

        let body = Self::buffered_body(&request);
        let url = self.build_model_url(&request.model);

        tracing::debug!(
            model = %request.model,
            url = %url,
            "Vertex Anthropic request"
        );

        let mut last_err = None;
        for attempt in 0..MAX_RETRIES {
            super::backoff::wait(attempt).await;
            let Some(text) = self
                .attempt(&url, &access_token, &body, attempt, &mut last_err)
                .await?
            else {
                continue;
            };
            return crate::response::parse(&text);
        }
        anyhow::bail!(
            "Vertex AI Anthropic request failed after {MAX_RETRIES} attempts: {}",
            last_err.unwrap_or_default()
        )
    }
}
