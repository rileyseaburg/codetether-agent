//! Embedding backend abstraction.
//!
//! [`TextEmbedder`] is the seam that lets callers swap the dependency-free
//! [`LocalEmbeddingEngine`](super::embed::LocalEmbeddingEngine) for a real,
//! learned embedding model (see [`provider`](super::provider)) without
//! changing the [`VectorStore`](super::store::VectorStore) call sites.

use super::embed::LocalEmbeddingEngine;
use super::vector::EmbeddingVector;
use anyhow::Result;
use async_trait::async_trait;

/// A backend that turns text into normalized embedding vectors.
///
/// The synchronous [`embed`](TextEmbedder::embed) covers local engines; the
/// async [`embed_batch`](TextEmbedder::embed_batch) lets network-backed
/// implementations amortize round-trips. The default `embed_batch` simply maps
/// `embed` so local backends need only implement one method.
#[async_trait]
pub trait TextEmbedder: Send + Sync {
    /// Embed a single text synchronously.
    fn embed(&self, input: &str) -> EmbeddingVector;

    /// Embed many texts, defaulting to per-item [`embed`](TextEmbedder::embed).
    async fn embed_batch(&self, inputs: &[String]) -> Result<Vec<EmbeddingVector>> {
        Ok(inputs.iter().map(|input| self.embed(input)).collect())
    }
}

#[async_trait]
impl TextEmbedder for LocalEmbeddingEngine {
    fn embed(&self, input: &str) -> EmbeddingVector {
        LocalEmbeddingEngine::embed(self, input)
    }
}
