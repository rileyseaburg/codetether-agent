//! Semantic similarity scoring for memory search.
//!
//! Uses cached per-entry embeddings from the local, file-based vector engine
//! so ranking captures meaning, not just shared tokens. No network calls.

use super::MemoryEntry;
use crate::vectordb::{EmbeddingVector, cosine};

/// Cosine similarity between a precomputed `query` vector and an entry.
///
/// Falls back to `0.0` when the entry has no cached embedding (its lexical
/// score still keeps it eligible). Embed the query once per search and reuse
/// it across all entries.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::memory::MemoryEntry;
/// use codetether_agent::tool::memory::semantic::score_against;
/// use codetether_agent::vectordb::{Embeddable, LocalEmbeddingEngine};
///
/// let engine = LocalEmbeddingEngine::default();
/// let mut entry = MemoryEntry::new("rust async tokio runtime", vec![]);
/// entry.ensure_embedding();
///
/// let strong = score_against(&entry, &engine.embed("tokio async runtime"));
/// let weak = score_against(&entry, &engine.embed("kubernetes pod scaling"));
/// assert!(strong > weak);
/// ```
pub fn score_against(entry: &MemoryEntry, query_vec: &EmbeddingVector) -> f32 {
    match entry.cached_embedding() {
        Some(vec) => cosine(vec, query_vec),
        None => 0.0,
    }
}
