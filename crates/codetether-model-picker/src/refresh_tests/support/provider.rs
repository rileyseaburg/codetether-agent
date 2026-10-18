//! Static model-list provider; completion operations are deliberately unused.
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::{
    CompletionRequest, CompletionResponse, EmbeddingRequest, EmbeddingResponse, ModelInfo,
    Provider, StreamChunk,
};
use futures::stream::BoxStream;
pub(in crate::refresh_tests) struct StaticProvider {
    pub(in crate::refresh_tests) name: &'static str,
    pub(in crate::refresh_tests) models: Vec<ModelInfo>,
}
#[async_trait]
impl Provider for StaticProvider {
    fn name(&self) -> &str {
        self.name
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        Ok(self.models.clone())
    }

    async fn complete(&self, _request: CompletionRequest) -> Result<CompletionResponse> {
        unimplemented!("not used by model picker tests")
    }

    async fn complete_stream(
        &self,
        _request: CompletionRequest,
    ) -> Result<BoxStream<'static, StreamChunk>> {
        unimplemented!("not used by model picker tests")
    }

    async fn embed(&self, _request: EmbeddingRequest) -> Result<EmbeddingResponse> {
        unimplemented!("not used by model picker tests")
    }
}
