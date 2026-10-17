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
    super::provider_impl_delegates::delegate_metrics_provider_basics!();

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        self.inner.list_models().await
    }

    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse> {
        super::calls::complete(self, request).await
    }

    async fn complete_scoped(
        &self,
        request: CompletionRequest,
        session_id: &str,
    ) -> Result<CompletionResponse> {
        super::calls::complete_scoped(self, request, session_id).await
    }

    async fn complete_stream(
        &self,
        request: CompletionRequest,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        super::calls::stream(self, request).await
    }

    async fn complete_stream_scoped(
        &self,
        request: CompletionRequest,
        session_id: &str,
    ) -> Result<futures::stream::BoxStream<'static, StreamChunk>> {
        super::calls::stream_scoped(self, request, session_id).await
    }

    async fn embed(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse> {
        self.inner.embed(request).await
    }
}
