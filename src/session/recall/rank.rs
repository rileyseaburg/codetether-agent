//! Hybrid lexical and local-vector ranking for recall evidence.

use super::hit::RecallHit;
use super::indexed_session::IndexedSession;

mod query;

const EMBEDDING_DIMENSIONS: usize = 96;
pub(super) use query::Query;

pub(super) fn search(
    sessions: &[IndexedSession],
    query: &str,
    excluded_session: Option<&str>,
    limit: usize,
    minimum_score: f32,
) -> Vec<RecallHit> {
    let Some(query) = Query::new(query, minimum_score) else {
        return Vec::new();
    };
    let mut hits = Vec::new();
    for session in sessions {
        merge(&mut hits, query.hits(session, excluded_session), limit);
    }
    hits
}

pub(super) fn merge(hits: &mut Vec<RecallHit>, mut incoming: Vec<RecallHit>, limit: usize) {
    hits.append(&mut incoming);
    hits.sort_by(|left, right| right.score.total_cmp(&left.score));
    hits.truncate(limit);
}
