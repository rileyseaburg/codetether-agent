//! Over-budget reporting after terminal truncation.

use tokio::sync::mpsc;
use uuid::Uuid;

use crate::session::SessionEvent;
use crate::session::event_compaction::{CompactionFailure, FallbackStrategy};

use super::enforce_events::emit;

/// Emit `CompactionFailed` when terminal truncation still leaves the
/// request over the safety budget; no-op otherwise.
pub(super) async fn emit_overrun_if_needed(
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
    trace_id: Uuid,
    after_tokens: usize,
    safety_budget: usize,
) {
    if after_tokens <= safety_budget {
        return;
    }
    tracing::error!(
        after_tokens,
        safety_budget,
        "Terminal truncation still over budget; request will likely fail at the provider"
    );
    emit(
        event_tx,
        SessionEvent::CompactionFailed(CompactionFailure {
            trace_id,
            fell_back_to: Some(FallbackStrategy::Truncate),
            reason: "terminal truncation still exceeds safety budget".to_string(),
            after_tokens,
            budget: safety_budget,
        }),
    )
    .await;
}
