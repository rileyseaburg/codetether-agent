//! One independently retrievable session-history range.

use serde::{Deserialize, Serialize};

use codetether_vectordb::EmbeddingVector;

/// Compact evidence retained for one contiguous message range.
/// # Examples
/// ```
/// use codetether_recall_index::RecallDocument;
/// let document = RecallDocument { start: 0, end: 1, excerpt: "hello".into(),
///     tokens: vec!["hello".into()],
///     embedding: codetether_vectordb::LocalEmbeddingEngine::new(96).embed("hello") };
/// assert_eq!(document.end, 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallDocument {
    /// Inclusive transcript message index.
    pub start: usize,
    /// Exclusive transcript message index.
    pub end: usize,
    /// Bounded source excerpt returned to callers.
    pub excerpt: String,
    /// Unique normalized terms used for lexical matching.
    pub tokens: Vec<String>,
    /// Cached local embedding used for semantic matching.
    pub embedding: EmbeddingVector,
}
