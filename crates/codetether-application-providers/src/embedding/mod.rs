//! Embedding selection using the application's initialized provider registry.
//!
//! [auto_embedder] preserves local/cloud/hash fallback selection. [AutoEmbedder]
//! supplies the same selection to memory tools, using the shared provider cache.
use crate::{Application, ProviderRegistry};
use codetether_vectordb::TextEmbedder;
use std::sync::Arc;
mod factory;
mod marker;
pub use marker::AutoEmbedder;

/// Select local or cloud embeddings, retaining the hashing fallback on failure.
/// # Arguments
/// `registry` contains the application's initialized providers.
/// # Returns
/// The selected embedder, or `None` to use the caller's hashing fallback.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_application_providers::Application>(registry: &codetether_application_providers::ProviderRegistry<H>) {
/// let _embedder = codetether_application_providers::embedding::auto_embedder(registry).await;
/// # }
/// ```
pub async fn auto_embedder<H: Application>(
    registry: &ProviderRegistry<H>,
) -> Option<Arc<dyn TextEmbedder>> {
    codetether_embedding_backends::auto::auto_embedder(registry).await
}

/// Select the first supported cloud embedding provider in its established order.
/// # Arguments
/// `registry` supplies initialized provider lookup.
/// # Returns
/// A cloud embedder, or `None` when none is supported.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_application_providers::Application>(registry: &codetether_application_providers::ProviderRegistry<H>) {
/// let _embedder = codetether_application_providers::embedding::cloud_embedder(registry);
/// # }
/// ```
pub fn cloud_embedder<H: Application>(
    registry: &ProviderRegistry<H>,
) -> Option<Arc<dyn TextEmbedder>> {
    codetether_embedding_backends::auto::provider_pick::cloud_embedder(registry)
}
