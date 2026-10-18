//! Delegate embedding requests to an application provider.

use anyhow::Result;
use codetether_indexer::EmbeddingProvider;
use std::sync::Arc;

pub(super) struct ApplicationProvider(pub(super) Arc<dyn codetether_provider_core::Provider>);

#[async_trait::async_trait]
impl EmbeddingProvider for ApplicationProvider {
    fn name(&self) -> &str {
        self.0.name()
    }
    async fn embed(
        &self,
        request: codetether_provider_core::EmbeddingRequest,
    ) -> Result<codetether_provider_core::EmbeddingResponse> {
        self.0.embed(request).await
    }
}
