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
    async fn complete(&self, mut request: CompletionRequest) -> Result<CompletionResponse> {
        if self.provider_name == "groq" {
            groq::validate(&request)?;
            groq_roles::prepare(&mut request);
            self.cap_groq_output(&mut request).await;
        }
        if self.is_lmstudio() {
            return self.local_complete(request).await;
        }
        self.complete_request(request).await
    }
    async fn complete_stream(
        &self,
        mut request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        if self.provider_name == "groq" {
            groq::validate(&request)?;
            groq_roles::prepare(&mut request);
            self.cap_groq_output(&mut request).await;
        }
        if self.is_lmstudio() {
            return self.local_stream(request).await;
        }
        self.stream_request(request).await
    }
    async fn embed(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse> {
        self.embed_request(request).await
    }
}
