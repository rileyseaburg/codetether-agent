//! Cached provider initialization and fallback for automatic memory embeddings.
use super::{Application, AutoEmbedder, ProviderRegistry, TextEmbedder, auto_embedder};
use std::sync::Arc;
#[async_trait::async_trait]
impl<H: Application> codetether_memory_tool::AutoEmbedder for AutoEmbedder<ProviderRegistry<H>> {
    async fn create() -> Option<Arc<dyn TextEmbedder>> {
        let registry = match ProviderRegistry::<H>::shared_from_vault().await {
            Ok(registry) => registry,
            Err(_) => Arc::new(ProviderRegistry::new()),
        };
        auto_embedder(&registry).await
    }
}
