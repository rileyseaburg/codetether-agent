//! Score one recall document against a prepared local query.

use crate::vectordb::{EmbeddingVector, cosine};

use super::document::RecallDocument;
use super::hit::RecallHit;
use super::indexed_session::IndexedSession;

const SEMANTIC_FLOOR: f32 = 0.45;

pub(super) fn hit(
    session: &IndexedSession,
    document: &RecallDocument,
    query_tokens: &[String],
    query_embedding: &EmbeddingVector,
    minimum_score: f32,
) -> Option<RecallHit> {
    let matched = query_tokens
        .iter()
        .filter(|token| document.tokens.binary_search(token).is_ok())
        .count();
    let lexical = matched as f32 / query_tokens.len() as f32;
    let semantic = cosine(query_embedding, &document.embedding).max(0.0);
    if matched == 0 && semantic < SEMANTIC_FLOOR {
        return None;
    }
    let score = lexical * 0.8 + semantic * 0.2;
    (score >= minimum_score).then(|| RecallHit {
        session_id: session.session_id.clone(),
        title: session.title.clone(),
        start: document.start,
        end: document.end,
        score,
        excerpt: document.excerpt.clone(),
    })
}
