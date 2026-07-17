//! Delegating [`Provider`] implementation for metrics instrumentation.

use super::super::{
    CompletionRequest, CompletionResponse, EmbeddingRequest, EmbeddingResponse, ModelInfo,
    Provider, StreamChunk,
};
use super::MetricsProvider;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
impl Provider for MetricsProvider {
    fn name(&self) -> &str {
        self.inner.name()
    }

    fn supports_structured_streaming(&self) -> bool {
        self.inner.supports_structured_streaming()
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.inner.list_models().await
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        super::calls::complete(self, request).await
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        super::calls::stream(self, request).await
    }

    async fn embed(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse> {
        self.inner.embed(request).await
    }
}
