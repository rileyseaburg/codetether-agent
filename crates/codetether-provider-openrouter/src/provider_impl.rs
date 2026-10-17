//! Provider contract delegation.
use crate::*;
use async_trait::async_trait;
#[async_trait]
impl Provider for OpenRouterProvider {
    fn name(&self) -> &str {
        "openrouter"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.discover_models().await
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.complete_buffered(request).await
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        self.start_stream(request).await
    }
}
