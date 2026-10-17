//! Provider trait delegation.
use crate::*;
#[async_trait]
impl Provider for CopilotProvider {
    fn name(&self) -> &str {
        &self.provider_name
    }
    fn supports_structured_streaming(&self) -> bool {
        false
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.models_inner().await
    }
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.complete_inner(request).await
    }
    async fn complete_stream(
        &self,
        _request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        anyhow::bail!(
            "Provider '{}' does not support structured streaming",
            self.name()
        )
    }
}
