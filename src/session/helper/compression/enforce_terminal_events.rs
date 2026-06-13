//! Event emission for the terminal-truncation path.

use tokio::sync::mpsc;
use uuid::Uuid;

use crate::session::SessionEvent;
use crate::session::event_compaction::{CompactionOutcome, ContextTruncation, FallbackStrategy};

use super::enforce_events::emit;
use super::enforce_terminal_overrun::emit_overrun_if_needed;

/// Emit `ContextTruncated`, `CompactionCompleted(Truncate)`, and — when
/// the request is still over budget — `CompactionFailed`.
pub(super) async fn emit_terminal_events(
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
    trace_id: Uuid,
    before_tokens: usize,
    after_tokens: usize,
    dropped_tokens: usize,
    kept_messages: usize,
    safety_budget: usize,
) {
    emit(
        event_tx,
        SessionEvent::ContextTruncated(ContextTruncation {
            trace_id,
            dropped_tokens,
            kept_messages,
            archive_ref: None,
        }),
    )
    .await;
    emit(
        event_tx,
        SessionEvent::CompactionCompleted(CompactionOutcome {
            trace_id,
            strategy: FallbackStrategy::Truncate,
            before_tokens,
            after_tokens,
            kept_messages,
        }),
    )
    .await;

    emit_overrun_if_needed(event_tx, trace_id, after_tokens, safety_budget).await;
}
