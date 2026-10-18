//! Shared compaction event delivery.
use super::event_send::emit;
use codetether_session_events::SessionEvent;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Emit a `CompactionStarted` event for `trace_id`.
pub(super) async fn emit_started<S>(
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
    trace_id: Uuid,
    reason: &str,
    before_tokens: usize,
    budget: usize,
) {
    emit(
        event_tx,
        SessionEvent::CompactionStarted(codetether_session_events::CompactionStart {
            trace_id,
            reason: reason.to_string(),
            before_tokens,
            budget,
        }),
    )
    .await;
}
