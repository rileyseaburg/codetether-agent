//! Shared compaction event delivery.
use super::event_send::emit;
use codetether_session_events::{CompactionOutcome, FallbackStrategy, SessionEvent};
use tokio::sync::mpsc;
use uuid::Uuid;

/// Emit a `CompactionCompleted(Rlm)` outcome event.
pub(super) async fn emit_completed<S>(
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
    trace_id: Uuid,
    before_tokens: usize,
    after_tokens: usize,
    kept_messages: usize,
) {
    emit(
        event_tx,
        SessionEvent::CompactionCompleted(CompactionOutcome {
            trace_id,
            strategy: FallbackStrategy::Rlm,
            before_tokens,
            after_tokens,
            kept_messages,
        }),
    )
    .await;
}
