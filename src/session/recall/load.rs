//! Direct recall sidecar loading.

use super::indexed_session::IndexedSession;

pub(super) async fn session(session_id: &str) -> Option<IndexedSession> {
    super::direct_load::session(session_id).await
}
