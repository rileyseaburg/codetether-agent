//! Delegate embedding requests to an application provider.

use super::*;
use std::sync::Arc;

pub(super) struct ApplicationProvider(pub(super) Arc<dyn crate::provider::Provider>);

#[async_trait::async_trait]
impl EmbeddingProvider for ApplicationProvider {
    fn name(&self) -> &str {
        self.0.name()
    }
    async fn embed(
        &self,
        request: crate::provider::EmbeddingRequest,
    ) -> Result<crate::provider::EmbeddingResponse> {
        self.0.embed(request).await
    }
}
