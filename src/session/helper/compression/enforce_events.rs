//! Compaction lifecycle event emission helpers.

use tokio::sync::mpsc;
use uuid::Uuid;

use crate::session::SessionEvent;
use crate::session::event_compaction::{CompactionOutcome, FallbackStrategy};

/// Emit a `CompactionCompleted(Rlm)` outcome event.
pub(super) async fn emit_completed(
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
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

/// Send `ev` when an event channel is present; drop it silently otherwise.
pub(super) async fn emit(event_tx: Option<&mpsc::Sender<SessionEvent>>, ev: SessionEvent) {
    if let Some(tx) = event_tx {
        let _ = tx.send(ev).await;
    }
}

/// Emit a `CompactionStarted` event for `trace_id`.
pub(super) async fn emit_started(
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
    trace_id: Uuid,
    reason: &str,
    before_tokens: usize,
    budget: usize,
) {
    emit(
        event_tx,
        SessionEvent::CompactionStarted(crate::session::event_compaction::CompactionStart {
            trace_id,
            reason: reason.to_string(),
            before_tokens,
            budget,
        }),
    )
    .await;
}
