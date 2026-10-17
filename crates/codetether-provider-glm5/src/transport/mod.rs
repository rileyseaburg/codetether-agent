//! Buffered completion orchestration.
use crate::*;
impl Glm5Provider {
    pub(crate) async fn buffered(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let (body, model) = Self::buffered_body(&request);
        tracing::debug!(model = %model, endpoint = %self.base_url, "GLM-5 FP8 request");

        let response = self
            .send(&body)
            .await
            .context("Failed to send request to GLM-5 FP8 endpoint")?;

        let text = read::read(response).await?;

        Self::parse(&text)
    }
}
mod read;
mod send;
