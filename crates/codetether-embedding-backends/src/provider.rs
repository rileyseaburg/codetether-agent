//! Provider-backed embedding implementation of [`TextEmbedder`].
//!
//! Wraps any [`Provider`](codetether_provider_core::Provider) that supports embeddings
//! (e.g. OpenAI `text-embedding-3-small`) so the vector store can use a real,
//! learned model. Network calls happen in the async
//! [`embed_batch`](TextEmbedder::embed_batch); the synchronous
//! [`embed`](TextEmbedder::embed) uses a bundled local engine so callers on a
//! sync path still get a usable vector without blocking a runtime.

use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::{EmbeddingRequest, Provider};
use codetether_vectordb::EmbeddingVector;
use codetether_vectordb::LocalEmbeddingEngine;
use codetether_vectordb::TextEmbedder;
use std::sync::Arc;

/// A [`TextEmbedder`] that calls a remote embedding model via a [`Provider`].
///
/// # Examples
/// ```
/// use codetether_embedding_backends::ProviderEmbedder;
/// use codetether_provider_core::Provider;
/// use std::sync::Arc;
/// fn create(provider: Arc<dyn Provider>) -> ProviderEmbedder {
///     ProviderEmbedder::new(provider, "text-embedding-3-small")
/// }
/// ```
pub struct ProviderEmbedder {
    provider: Arc<dyn Provider>,
    model: String,
    local: LocalEmbeddingEngine,
}

impl ProviderEmbedder {
    /// Build an embedder for `model` served by `provider`.
    /// The synchronous path retains a local hashing engine.
    pub fn new(provider: Arc<dyn Provider>, model: impl Into<String>) -> Self {
        Self {
            provider,
            model: model.into(),
            local: LocalEmbeddingEngine::default(),
        }
    }
}

#[async_trait]
impl TextEmbedder for ProviderEmbedder {
    fn embed(&self, input: &str) -> EmbeddingVector {
        self.local.embed(input)
    }

    async fn embed_batch(&self, inputs: &[String]) -> Result<Vec<EmbeddingVector>> {
        if inputs.is_empty() {
            return Ok(Vec::new());
        }
        let request = EmbeddingRequest {
            model: self.model.clone(),
            inputs: inputs.to_vec(),
        };
        let response = self.provider.embed(request).await?;
        Ok(response
            .embeddings
            .into_iter()
            .map(EmbeddingVector::new)
            .collect())
    }
}
