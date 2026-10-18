//! Create a compaction trace and announce its initial budget.
use super::{enforce_budget::BudgetCheck, enforce_events::emit_started};
use codetether_session_events::SessionEvent;
use tokio::sync::mpsc;
use uuid::Uuid;
pub(super) async fn start<S>(
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
    trigger_reason: &str,
    check: &BudgetCheck,
) -> Uuid {
    let trace_id = Uuid::new_v4();
    emit_started(
        event_tx,
        trace_id,
        trigger_reason,
        check.initial_est,
        check.safety_budget,
    )
    .await;

    trace_id
}
