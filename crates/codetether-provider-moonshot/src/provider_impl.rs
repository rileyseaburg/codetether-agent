//! Provider trait and unsupported structured-streaming response.
use crate::*;
use async_trait::async_trait;
#[async_trait]
impl Provider for MoonshotProvider {
    fn name(&self) -> &str {
        "moonshotai"
    }

    fn supports_structured_streaming(&self) -> bool {
        false
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(crate::catalog::models())
    }
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        let body = Self::build_body(&request);
        tracing::debug!("Moonshot request to model {}", request.model);
        crate::response::parse(self.send_body(&body).await?)
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        tracing::debug!(
            provider = "moonshotai",
            model = %request.model,
            message_count = request.messages.len(),
            "Structured streaming is not implemented"
        );
        anyhow::bail!(
            "Provider '{}' does not support structured streaming",
            self.name()
        )
    }
}
