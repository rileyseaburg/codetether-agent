//! Cache-aware recall sidecar loading.

use futures::StreamExt;
use std::path::Path;
use std::sync::Arc;

use super::indexed_session::IndexedSession;

const READ_CONCURRENCY: usize = 16;

pub(super) async fn workspace(workspace: &Path) -> Arc<Vec<IndexedSession>> {
    super::backfill::schedule(workspace);
    let Ok(key) = super::paths::catalog(workspace) else {
        return Arc::new(Vec::new());
    };
    if let Some(sessions) = super::cache::get(&key) {
        return sessions;
    }
    let _guard = super::store_lock::acquire().await;
    if let Some(sessions) = super::cache::get(&key) {
        return sessions;
    }
    let canonical = super::paths::canonical(workspace);
    let catalog = super::catalog_io::read(&canonical).await;
    let sessions = futures::stream::iter(catalog.session_ids)
        .map(|id| async move { super::session_io::read(&id).await })
        .buffer_unordered(READ_CONCURRENCY)
        .filter_map(|session| async move { session })
        .filter(|session| {
            let matches = session.workspace == canonical;
            async move { matches }
        })
        .collect()
        .await;
    super::cache::insert(key, sessions)
}

pub(super) async fn session(session_id: &str) -> Option<IndexedSession> {
    super::direct_load::session(session_id).await
}
