//! Stream workspace sidecars with a fixed bound on concurrent reads.
use codetether_recall_index::{Query, RecallHit};
use futures::StreamExt;
use std::path::Path;
const READ_CONCURRENCY: usize = 16;

pub(super) async fn run<H: crate::Host>(
    workspace: &Path,
    query: &Query,
    excluded_session: Option<&str>,
    limit: usize,
    host: &H,
) -> (Vec<RecallHit>, usize) {
    host.schedule_backfill(workspace);
    let canonical = codetether_recall_store::paths::canonical(workspace);
    let catalog = codetether_recall_store::read_catalog(&canonical, host).await;
    let mut sidecars = futures::stream::iter(catalog.session_ids)
        .map(|id| async move { codetether_recall_store::read(&id, host).await })
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
        codetether_recall_index::merge(&mut hits, incoming, limit);
    }
    (hits, scanned)
}
