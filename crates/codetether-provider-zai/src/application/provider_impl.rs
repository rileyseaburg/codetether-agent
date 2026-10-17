//! Provider contract delegation for the application profile.
use super::ZaiProvider;
use crate::{
    CompletionRequest, CompletionResponse, ModelInfo, StreamChunk, catalog as zai_catalog,
};
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::Provider;
#[async_trait]
impl Provider for ZaiProvider {
    fn name(&self) -> &str {
        "zai"
    }

    async fn list_models(&self) -> Result<Vec<ModelInfo>> {
        // Attempt dynamic model discovery from the Z.AI /models endpoint.
        // When the API is reachable, this returns the authoritative model
        // catalog including newly-released models without a code change.
        let discovered = self.discover_models_from_api().await;
        Ok(zai_catalog::merge_known(discovered))
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
