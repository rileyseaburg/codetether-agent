//! Application memory aliases and automatic embedding-provider selection.
pub use codetether_memory_tool::{MemoryEntry, MemoryStats, fusion, query_match, search, semantic};
/// Store using the application's existing automatic embedding selection.
pub type MemoryStore = codetether_memory_tool::Store<ApplicationAutoEmbedder>;
/// Memory tool using the application's existing provider selection.
pub type MemoryTool = codetether_memory_tool::Runtime<ApplicationAutoEmbedder>;
/// Factory preserving the application-owned provider registry and embedding policy.
///
/// # Examples
/// ```rust
/// let _factory = codetether_agent::tool::memory::ApplicationAutoEmbedder;
/// ```
#[derive(Debug, Clone, Default)]
pub struct ApplicationAutoEmbedder;
#[async_trait::async_trait]
impl codetether_memory_tool::AutoEmbedder for ApplicationAutoEmbedder {
    async fn create() -> Option<std::sync::Arc<dyn crate::vectordb::TextEmbedder>> {
        use crate::provider::ProviderRegistry;
        use crate::vectordb::auto::auto_embedder;
        let registry = match ProviderRegistry::shared_from_vault().await {
            Ok(registry) => registry,
            Err(_) => std::sync::Arc::new(ProviderRegistry::new()),
        };
        auto_embedder(&registry).await
    }
}
