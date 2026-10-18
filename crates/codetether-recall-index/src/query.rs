//! One prepared recall query reused across streamed sidecars.

use codetether_vectordb::{EmbeddingVector, LocalEmbeddingEngine};

use crate::{IndexedSession, RecallHit};

/// Tokenized and embedded query reused while sidecars stream through memory.
/// # Examples
/// ```
/// use codetether_recall_index::Query;
/// assert!(Query::new("deployment policy", 0.2).is_some());
/// assert!(Query::new("the and", 0.2).is_none());
/// ```
pub struct Query {
    tokens: Vec<String>,
    embedding: EmbeddingVector,
    minimum_score: f32,
}

impl Query {
    /// Prepare a query, returning `None` when it has no searchable terms.
    pub fn new(input: &str, minimum_score: f32) -> Option<Self> {
        let tokens = super::tokens::query(input);
        (!tokens.is_empty()).then(|| Self {
            tokens,
            embedding: LocalEmbeddingEngine::new(super::rank::EMBEDDING_DIMENSIONS).embed(input),
            minimum_score,
        })
    }

    /// Rank one decoded session without retaining the session afterward.
    pub fn hits(&self, session: &IndexedSession, excluded_session: Option<&str>) -> Vec<RecallHit> {
        if excluded_session == Some(session.session_id.as_str()) {
            return Vec::new();
        }
        session
            .documents
            .iter()
            .filter_map(|document| {
                super::rank_score::hit(
                    session,
                    document,
                    &self.tokens,
                    &self.embedding,
                    self.minimum_score,
                )
            })
            .collect()
    }
}
