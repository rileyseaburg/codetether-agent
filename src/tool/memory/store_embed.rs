//! Embedding-backend-aware operations for [`MemoryStore`](super::MemoryStore).
//!
//! These mirror `add`/`search` but route through the optional learned-model
//! [`TextEmbedder`](crate::vectordb::TextEmbedder), keeping query and entry
//! vectors in the same embedding space.
//!
//! When the learned-model backend is unavailable or fails, semantic ranking is
//! skipped entirely (query vector is `None`) so ranking falls back to lexical
//! scoring only — never mixing vector spaces.

use super::embedder_handle::EmbedderHandle;
use super::{MemoryEntry, MemoryStore, scope_widen};
use crate::vectordb::{Embeddable, TextEmbedder};
use std::sync::Arc;

impl MemoryStore {
    /// Attach a learned-model embedding backend.
    pub fn with_embedder(mut self, embedder: Arc<dyn TextEmbedder>) -> Self {
        self.embedder = EmbedderHandle(Some(embedder));
        self
    }

    /// Add a memory, computing its vector with the configured backend.
    ///
    /// If no backend is set or the call fails, the entry is stored without an
    /// embedding and will be ranked by lexical score only.
    pub async fn add_embedded(&mut self, mut entry: MemoryEntry) -> String {
        if entry.embedding.is_none()
            && let Some(Ok(mut vecs)) = self.embedder.embed_batch(&[entry.embedding_text()]).await
            && let Some(vec) = vecs.pop()
        {
            entry.embedding = Some(vec);
        }
        self.add(entry)
    }

    /// Search with the configured backend for semantic ranking.
    ///
    /// If the backend is unavailable or returns an error the query vector is
    /// `None` and ranking falls back to lexical-only — never mixing a
    /// hash-space query against learned-space entry vectors.
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
                // Backend absent or failed: skip semantic ranking entirely.
                _ => None,
            },
            None => None,
        };
        scope_widen::run(
            &mut self.entries,
            query,
            tags,
            scope,
            query_vec.as_ref(),
            limit,
        )
    }
}
