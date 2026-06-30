//! Trait for types that can be embedded into the vector store.

use super::embed::LocalEmbeddingEngine;
use super::vector::EmbeddingVector;

/// Implement on a payload type to define how it is turned into embedding text.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::vectordb::{Embeddable, LocalEmbeddingEngine};
///
/// struct Note { title: String, body: String }
/// impl Embeddable for Note {
///     fn embedding_text(&self) -> String {
///         format!("{} {}", self.title, self.body)
///     }
/// }
///
/// let engine = LocalEmbeddingEngine::new(32);
/// let note = Note { title: "t".into(), body: "b".into() };
/// assert_eq!(note.embed_with(&engine).len(), 32);
/// ```
pub trait Embeddable {
    /// The text used to compute this item's embedding.
    fn embedding_text(&self) -> String;

    /// Compute the embedding using `engine`.
    fn embed_with(&self, engine: &LocalEmbeddingEngine) -> EmbeddingVector {
        engine.embed(&self.embedding_text())
    }
}
