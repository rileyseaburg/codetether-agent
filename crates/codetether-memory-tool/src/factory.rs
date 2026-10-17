//! Caller-owned automatic embedding backend selection.
use async_trait::async_trait;
use codetether_vectordb::TextEmbedder;
use std::sync::Arc;
/// Selects an optional embedding backend without moving provider ownership.
///
/// # Implementors
/// Application adapters may choose local or provider-backed embedding engines.
#[async_trait]
pub trait AutoEmbedder: Clone + std::fmt::Debug + Default + Send + Sync + 'static {
    /// Select a backend, or retain deterministic local embeddings with None.
    async fn create() -> Option<Arc<dyn TextEmbedder>>;
}
/// Factory for standalone memory tools that do not perform backend discovery.
///
/// # Examples
/// ```rust
/// let _factory = codetether_memory_tool::NoAutoEmbedder;
/// ```
#[derive(Debug, Clone, Default)]
pub struct NoAutoEmbedder;
#[async_trait]
impl AutoEmbedder for NoAutoEmbedder {
    async fn create() -> Option<Arc<dyn TextEmbedder>> {
        None
    }
}
