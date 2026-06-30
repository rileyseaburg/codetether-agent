//! Cloud embedding-provider selection from the provider registry.

use crate::provider::ProviderRegistry;
use crate::vectordb::{ProviderEmbedder, TextEmbedder};
use std::sync::Arc;

/// Providers known to support embeddings, with their default model, in
/// preference order.
const EMBEDDING_PROVIDERS: &[(&str, &str)] = &[
    ("openai", "text-embedding-3-small"),
    ("google", "text-embedding-004"),
];

/// Build a [`ProviderEmbedder`] from the first available embedding provider.
///
/// Returns `None` when no known embedding provider is registered.
pub fn cloud_embedder(registry: &ProviderRegistry) -> Option<Arc<dyn TextEmbedder>> {
    for (name, model) in EMBEDDING_PROVIDERS {
        if let Some(provider) = registry.get(name) {
            let embedder = ProviderEmbedder::new(provider, *model);
            return Some(Arc::new(embedder));
        }
    }
    None
}
