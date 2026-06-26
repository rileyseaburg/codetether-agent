//! Embedding derivation for [`MemoryEntry`](super::MemoryEntry).

use super::MemoryEntry;
use crate::vectordb::{Embeddable, EmbeddingVector, LocalEmbeddingEngine};

impl Embeddable for MemoryEntry {
    fn embedding_text(&self) -> String {
        format!("{} {}", self.content, self.tags.join(" "))
    }
}

impl MemoryEntry {
    /// Compute and cache this entry's embedding if it is not already present.
    ///
    /// Embedding once on save (rather than per query) keeps search O(n) in
    /// cheap dot products instead of O(n) re-embeddings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::tool::memory::MemoryEntry;
    ///
    /// let mut entry = MemoryEntry::new("rust async runtime", vec![]);
    /// assert!(entry.embedding.is_none());
    /// entry.ensure_embedding();
    /// assert!(entry.embedding.is_some());
    /// ```
    pub fn ensure_embedding(&mut self) {
        if self.embedding.is_none() {
            self.embedding = Some(self.embed_with(&LocalEmbeddingEngine::default()));
        }
    }

    /// Borrow the cached embedding, computing nothing.
    pub fn cached_embedding(&self) -> Option<&EmbeddingVector> {
        self.embedding.as_ref()
    }
}
