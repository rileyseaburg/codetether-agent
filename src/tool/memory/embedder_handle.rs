//! A serialization-friendly handle to an optional embedding backend.
//!
//! [`MemoryStore`](super::MemoryStore) derives `Serialize`/`Debug`/`Clone`/
//! `Default`, none of which a bare `dyn TextEmbedder` satisfies. This wrapper
//! provides manual impls and is skipped during (de)serialization, so the chosen
//! backend is a runtime concern that never touches the on-disk format.

use crate::vectordb::TextEmbedder;
use std::sync::Arc;

/// Optional, swappable embedding backend for a memory store.
#[derive(Clone, Default)]
pub struct EmbedderHandle(pub Option<Arc<dyn TextEmbedder>>);

impl EmbedderHandle {
    /// Embed `inputs` via the configured backend, or `None` if unset.
    pub async fn embed_batch(
        &self,
        inputs: &[String],
    ) -> Option<anyhow::Result<Vec<crate::vectordb::EmbeddingVector>>> {
        match &self.0 {
            Some(embedder) => Some(embedder.embed_batch(inputs).await),
            None => None,
        }
    }
}

impl std::fmt::Debug for EmbedderHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = if self.0.is_some() { "set" } else { "unset" };
        write!(f, "EmbedderHandle({state})")
    }
}
