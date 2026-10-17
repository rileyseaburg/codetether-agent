//! Provider trait delegation.
use crate::*;
#[async_trait]
impl Provider for Glm5Provider {
    fn name(&self) -> &str {
        "glm5"
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        catalog::list_models().await
    }
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.buffered(request).await
    }
    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        self.streaming(request).await
    }
}
