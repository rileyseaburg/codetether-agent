//! Embedding-backend-aware operations for [`MemoryStore`](super::MemoryStore).
//!
//! These mirror `add`/`search` but route through the optional learned-model
//! [`TextEmbedder`](crate::vectordb::TextEmbedder), keeping query and entry
//! vectors in the same embedding space. All fall back to the local engine.

use super::embedder_handle::EmbedderHandle;
use super::{MemoryEntry, MemoryStore, search};
use crate::vectordb::{Embeddable, LocalEmbeddingEngine, TextEmbedder};
use std::sync::Arc;

impl MemoryStore {
    /// Attach a learned-model embedding backend.
    pub fn with_embedder(mut self, embedder: Arc<dyn TextEmbedder>) -> Self {
        self.embedder = EmbedderHandle(Some(embedder));
        self
    }

    /// Add a memory, preferring the configured embedder for its vector.
    ///
    /// Falls back to the local hashing engine when no backend is set or the
    /// remote call fails, so a save never blocks on the network.
    pub async fn add_embedded(&mut self, mut entry: MemoryEntry) -> String {
        if entry.embedding.is_none()
            && let Some(Ok(mut vecs)) = self.embedder.embed_batch(&[entry.embedding_text()]).await
            && let Some(vec) = vecs.pop()
        {
            entry.embedding = Some(vec);
        }
        self.add(entry)
    }

    /// Search, embedding the query with the configured backend for ranking.
    ///
    /// Ensures the query and entry vectors share an embedding space; falls back
    /// to the local engine when no backend is set or the remote call fails.
    pub async fn search_embedded(
        &mut self,
        query: Option<&str>,
        tags: Option<&[String]>,
        scope: Option<&str>,
        limit: usize,
    ) -> Vec<MemoryEntry> {
        let query_vec = match query.filter(|q| !q.trim().is_empty()) {
            Some(q) => match self.embedder.embed_batch(&[q.to_string()]).await {
                Some(Ok(mut v)) => v.pop(),
                _ => Some(LocalEmbeddingEngine::default().embed(q)),
            },
            None => None,
        };
        let candidates = search::collect(&mut self.entries, query, tags, scope);
        search::rank(candidates, query_vec.as_ref(), limit)
    }
}
