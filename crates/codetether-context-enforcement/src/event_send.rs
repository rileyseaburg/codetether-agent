//! Shared compaction event delivery.
use codetether_session_events::SessionEvent;
use tokio::sync::mpsc;

/// Send `ev` when an event channel is present; drop it silently otherwise.
pub(super) async fn emit<S>(event_tx: Option<&mpsc::Sender<SessionEvent<S>>>, ev: SessionEvent<S>) {
    if let Some(tx) = event_tx {
        let _ = tx.send(ev).await;
    }
}
