//! Narrow interfaces for resolving and invoking remote embedding providers.

use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::{EmbeddingRequest, EmbeddingResponse};
use std::sync::Arc;

/// A provider that implements the embedding operation used by the indexer.
///
/// # Implementors
/// Application adapters forward these calls to their selected provider.
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    /// Return the provider identifier recorded in index metadata.
    fn name(&self) -> &str;
    /// Embed an ordered batch of source-document descriptions.
    ///
    /// # Errors
    /// Return the provider error; the indexer applies configured transient retries.
    async fn embed(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse>;
}

/// Resolve a model selector without coupling indexing to credential loading.
///
/// # Implementors
/// Applications select providers and credentials. The standalone entry point
/// supports local embeddings without a resolver.
#[async_trait]
pub trait EmbeddingResolver: Send + Sync {
    /// Resolve `selector` into an embedding provider and its model identifier.
    ///
    /// # Errors
    /// Return a credential, provider-selection, or unsupported-model error.
    async fn resolve(&self, selector: &str) -> Result<(Arc<dyn EmbeddingProvider>, String)>;
}
