//! Provider trait delegation.
use crate::*;
use async_trait::async_trait;
#[async_trait]
impl Provider for VertexAnthropicProvider {
    fn name(&self) -> &str {
        "vertex-anthropic"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(vertex_anthropic_catalog::catalog())
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        self.complete_request(request).await
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        self.stream_request(request).await
    }
}
