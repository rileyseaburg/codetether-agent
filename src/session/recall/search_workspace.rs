//! Bounded-memory streaming search across workspace recall sidecars.

use futures::StreamExt;
use std::path::Path;

use super::hit::RecallHit;
use super::rank::Query;

const READ_CONCURRENCY: usize = 16;

pub(super) async fn run(
    workspace: &Path,
    query: &Query,
    excluded_session: Option<&str>,
    limit: usize,
) -> (Vec<RecallHit>, usize) {
    super::backfill::schedule(workspace);
    let canonical = super::paths::canonical(workspace);
    let catalog = super::catalog_io::read(&canonical).await;
    let mut sidecars = futures::stream::iter(catalog.session_ids)
        .map(|id| async move { super::session_io::read(&id).await })
        .buffer_unordered(READ_CONCURRENCY);
    let mut hits = Vec::new();
    let mut scanned = 0;
    while let Some(session) = sidecars.next().await {
        let Some(session) = session else {
            continue;
        };
        if session.workspace != canonical {
            continue;
        }
        scanned += 1;
        let incoming = query.hits(&session, excluded_session);
        super::rank::merge(&mut hits, incoming, limit);
    }
    (hits, scanned)
}
