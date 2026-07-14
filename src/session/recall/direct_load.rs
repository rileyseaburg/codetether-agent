//! One-time direct-session migration when a sidecar is absent.

use super::indexed_session::IndexedSession;

pub(super) async fn session(session_id: &str) -> Option<IndexedSession> {
    if let Some(indexed) = super::session_io::read(session_id).await {
        return Some(indexed);
    }
    let session = crate::session::Session::load(session_id).await.ok()?;
    let indexed = tokio::task::spawn_blocking(move || super::build::session(&session))
        .await
        .ok()??;
    if let Err(error) = super::store::upsert(indexed.clone()).await {
        tracing::warn!(%error, %session_id, "direct recall sidecar migration failed");
    }
    Some(indexed)
}
