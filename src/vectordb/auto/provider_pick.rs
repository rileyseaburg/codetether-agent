//! Shared cloud embedding selection using application provider lookup.
use crate::provider::ProviderRegistry;
use crate::vectordb::TextEmbedder;
use std::sync::Arc;
/// Select the first supported embedding provider in its established order.
pub fn cloud_embedder(registry: &ProviderRegistry) -> Option<Arc<dyn TextEmbedder>> {
    codetether_embedding_backends::auto::provider_pick::cloud_embedder(registry)
}
