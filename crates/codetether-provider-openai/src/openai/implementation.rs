//! Shared provider contract implementation.

use super::*;

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> &str {
        &self.provider_name
    }
    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.list_provider_models().await
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
    async fn embed(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse> {
        self.embed_request(request).await
    }
}
