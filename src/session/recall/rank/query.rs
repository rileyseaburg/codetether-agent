//! One prepared recall query reused across streamed sidecars.

use crate::vectordb::{EmbeddingVector, LocalEmbeddingEngine};

use super::super::{hit::RecallHit, indexed_session::IndexedSession};

/// Tokenized and embedded query reused while sidecars stream through memory.
pub(crate) struct Query {
    tokens: Vec<String>,
    embedding: EmbeddingVector,
    minimum_score: f32,
}

impl Query {
    /// Prepare a query, returning `None` when it has no searchable terms.
    pub(crate) fn new(input: &str, minimum_score: f32) -> Option<Self> {
        let tokens = super::super::tokens::query(input);
        (!tokens.is_empty()).then(|| Self {
            tokens,
            embedding: LocalEmbeddingEngine::new(super::EMBEDDING_DIMENSIONS).embed(input),
            minimum_score,
        })
    }

    /// Rank one decoded session without retaining the session afterward.
    pub(crate) fn hits(
        &self,
        session: &IndexedSession,
        excluded_session: Option<&str>,
    ) -> Vec<RecallHit> {
        if excluded_session == Some(session.session_id.as_str()) {
            return Vec::new();
        }
        session
            .documents
            .iter()
            .filter_map(|document| {
                super::super::rank_score::hit(
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
