//! Reuse existing sidecars before one-time bounded direct-session migration.
use codetether_recall_index::IndexedSession;

pub(super) async fn session<H: crate::Host>(session_id: &str, host: &H) -> Option<IndexedSession> {
    if let Some(indexed) = codetether_recall_store::read(session_id, host).await {
        return Some(indexed);
    }
    let session = host.load_tail(session_id, 512).await.ok()?;
    let indexed = tokio::task::spawn_blocking(move || H::build(&session))
        .await
        .ok()??;
    if let Err(error) = codetether_recall_store::upsert(indexed.clone(), host).await {
        tracing::warn!(target: "codetether_agent::session::recall::direct_load",%error, %session_id, "direct recall sidecar migration failed");
    }
    Some(indexed)
}
