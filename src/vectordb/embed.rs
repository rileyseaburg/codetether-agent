//! Dependency-free local embedding engine (hashed token features).
//!
//! Produces deterministic, L2-normalized vectors without any network calls,
//! mirroring the hashing trick used by the codebase indexer.

use super::embed_hash::{accumulate_char_ngrams, accumulate_token};
use super::tokenize::tokenize;
use super::vector::EmbeddingVector;

/// Default embedding dimensionality.
pub const DEFAULT_DIMENSIONS: usize = 384;

/// Hashing-based embedding engine over a fixed dimensionality.
#[derive(Debug, Clone)]
pub struct LocalEmbeddingEngine {
    dimensions: usize,
}

impl Default for LocalEmbeddingEngine {
    fn default() -> Self {
        Self::new(DEFAULT_DIMENSIONS)
    }
}

impl LocalEmbeddingEngine {
    /// Create an engine producing `dimensions`-length vectors.
    pub fn new(dimensions: usize) -> Self {
        Self {
            dimensions: dimensions.max(1),
        }
    }

    /// Embed a single text into a normalized vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::vectordb::LocalEmbeddingEngine;
    ///
    /// let engine = LocalEmbeddingEngine::new(64);
    /// let v = engine.embed("hello world");
    /// assert_eq!(v.len(), 64);
    /// ```
    pub fn embed(&self, input: &str) -> EmbeddingVector {
        let mut vector = vec![0.0f32; self.dimensions];
        let tokens = tokenize(input);

        if tokens.is_empty() {
            accumulate_char_ngrams(&mut vector, input);
        } else {
            for (idx, token) in tokens.iter().enumerate() {
                let weight = 1.0f32 / (1.0 + (idx as f32 / 128.0));
                accumulate_token(&mut vector, token, weight);
                if let Some(next) = tokens.get(idx + 1) {
                    let bigram = format!("{token} {next}");
                    accumulate_token(&mut vector, &bigram, weight * 0.65);
                }
            }
        }
        EmbeddingVector::new(vector)
    }
}
