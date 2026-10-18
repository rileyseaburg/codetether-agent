//! Automatic embeddings bound to the application's initialized providers.
use crate::provider::ProviderRegistry;
use crate::vectordb::TextEmbedder;
#[cfg(feature = "candle")]
pub use codetether_embedding_backends::auto::{bert_embedder, bert_forward, local_pick};
pub use codetether_embedding_backends::auto::{catalog, hf_download, resources};
use std::sync::Arc;
pub mod provider_pick;
mod registry;

/// Select local or cloud embeddings, retaining the hashing fallback on failure.
pub async fn auto_embedder(registry: &ProviderRegistry) -> Option<Arc<dyn TextEmbedder>> {
    codetether_embedding_backends::auto::auto_embedder(registry).await
}
