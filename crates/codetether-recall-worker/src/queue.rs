//! Restore or mark deletion guards around queue lifecycle transitions.

/// Coalesce a pending session ID and launch its worker if idle.
/// # Arguments
/// Receives the session ID and shared native worker host.
/// # Returns
/// Returns after queueing; the worker continues asynchronously.
/// # Examples
/// ```no_run
/// # fn example(host: std::sync::Arc<impl codetether_recall_worker::Host>) {
/// codetether_recall_worker::schedule("session-id", host);
/// # }
/// ```
pub fn schedule<H: crate::Host>(session_id: &str, host: std::sync::Arc<H>) {
    codetether_recall_store::tombstone::restore(session_id);
    let spawn = super::queue_state::enqueue(session_id);
    if spawn {
        super::worker::spawn(session_id.to_string(), host);
    }
}

/// Mark deletion, discard queued work, then remove persisted recall data.
/// # Arguments
/// Receives the session ID and native worker host.
/// # Returns
/// Completes after shared recall-store removal.
/// # Errors
/// Propagates native path validation and persistence failures.
/// # Examples
/// ```no_run
/// # async fn example(host: &impl codetether_recall_worker::Host) -> anyhow::Result<()> {
/// codetether_recall_worker::remove("session-id", host).await
/// # }
/// ```
pub async fn remove<H: crate::Host>(session_id: &str, host: &H) -> anyhow::Result<()> {
    codetether_recall_store::tombstone::mark(session_id);
    super::queue_state::remove(session_id);
    codetether_recall_store::remove(session_id, host).await
}
