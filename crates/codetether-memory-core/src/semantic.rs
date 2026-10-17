//! Semantic similarity scoring for memory search.
//!
//! Uses cached per-entry embeddings from the local, file-based vector engine
//! so ranking captures meaning, not just shared tokens. No network calls.

use super::MemoryEntry;
use codetether_vectordb::{EmbeddingVector, cosine};

/// Cosine similarity between a precomputed `query` vector and an entry.
///
/// Falls back to `0.0` when the entry has no cached embedding (its lexical
/// score still keeps it eligible). Embed the query once per search and reuse
/// it across all entries.
///
/// # Examples
///
/// ```rust
/// use codetether_memory_core::MemoryEntry;
/// use codetether_memory_core::semantic::score_against;
/// use codetether_vectordb::{Embeddable, LocalEmbeddingEngine};
///
/// let engine = LocalEmbeddingEngine::default();
/// let mut entry = MemoryEntry::new("rust async tokio runtime", vec![]);
/// entry.ensure_embedding();
///
/// let strong = score_against(&entry, &engine.embed("tokio async runtime"));
/// let weak = score_against(&entry, &engine.embed("kubernetes pod scaling"));
/// assert!(strong > weak);
/// ```
///
/// # Arguments
/// `entry` supplies a cached vector; `query_vec` is the query vector.
/// # Returns
/// Cosine similarity, or zero when the entry has no embedding.
pub fn score_against(entry: &MemoryEntry, query_vec: &EmbeddingVector) -> f32 {
    match entry.cached_embedding() {
        Some(vec) => cosine(vec, query_vec),
        None => 0.0,
    }
}
