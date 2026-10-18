//! Hybrid lexical and local-vector ranking for recall evidence.

use super::indexed_session::IndexedSession;
use crate::RecallHit;

pub(super) const EMBEDDING_DIMENSIONS: usize = 96;
use crate::Query;

/// Search already loaded projections and retain their best scoring evidence.
/// # Arguments
/// Receives projections, query text, an optional excluded session, hit limit
/// and inclusive minimum score.
/// # Returns
/// Hits ordered by descending score, bounded by `limit`.
/// # Examples
/// ```
/// assert!(codetether_recall_index::search(&[], "policy", None, 3, 0.2).is_empty());
/// ```
pub fn search(
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

/// Merge hits while preserving descending score order and the requested limit.
/// # Arguments
/// Receives accumulated hits, incoming hits and the maximum retained count.
/// # Returns
/// Updates the accumulated vector in place.
/// # Examples
/// ```
/// let mut hits = vec![];
/// codetether_recall_index::merge(&mut hits, vec![], 3);
/// assert!(hits.is_empty());
/// ```
pub fn merge(hits: &mut Vec<RecallHit>, mut incoming: Vec<RecallHit>, limit: usize) {
    hits.append(&mut incoming);
    hits.sort_by(|left, right| right.score.total_cmp(&left.score));
    hits.truncate(limit);
}
