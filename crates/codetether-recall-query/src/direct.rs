//! Rank one directly requested session, migrating its sidecar if necessary.
use codetether_recall_index::{Query, RecallHit};

pub(super) async fn search<H: crate::Host>(
    id: &str,
    query: &Query,
    excluded: Option<&str>,
    limit: usize,
    host: &H,
) -> (Vec<RecallHit>, usize) {
    let Some(session) = super::migrate::session(id, host).await else {
        return (Vec::new(), 0);
    };
    let mut hits = Vec::new();
    codetether_recall_index::merge(&mut hits, query.hits(&session, excluded), limit);
    (hits, 1)
}
