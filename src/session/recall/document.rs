//! One independently retrievable session-history range.

use serde::{Deserialize, Serialize};

use crate::vectordb::EmbeddingVector;

/// Compact evidence retained for one contiguous message range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RecallDocument {
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
