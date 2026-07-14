//! Hybrid lexical and local-vector ranking for recall evidence.

use crate::vectordb::LocalEmbeddingEngine;

use super::hit::RecallHit;
use super::indexed_session::IndexedSession;

const EMBEDDING_DIMENSIONS: usize = 96;
pub(super) fn search(
    sessions: &[IndexedSession],
    query: &str,
    excluded_session: Option<&str>,
    limit: usize,
    minimum_score: f32,
) -> Vec<RecallHit> {
    let query_tokens = super::tokens::query(query);
    if query_tokens.is_empty() {
        return Vec::new();
    }
    let query_embedding = LocalEmbeddingEngine::new(EMBEDDING_DIMENSIONS).embed(query);
    let mut hits: Vec<_> = sessions
        .iter()
        .filter(|session| excluded_session != Some(session.session_id.as_str()))
        .flat_map(|session| {
            session.documents.iter().filter_map(|document| {
                super::rank_score::hit(
                    session,
                    document,
                    &query_tokens,
                    &query_embedding,
                    minimum_score,
                )
            })
        })
        .collect();
    hits.sort_by(|left, right| right.score.total_cmp(&left.score));
    hits.truncate(limit);
    hits
}
