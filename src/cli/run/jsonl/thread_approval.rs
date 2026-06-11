//! JSONL conversion for approval thread events.

use crate::session::thread_store::ThreadEvent;

use super::{event::RunEvent, thread::field};

pub(super) fn to_run_event(event: &ThreadEvent) -> Option<RunEvent<'_>> {
    match event.kind.as_str() {
        "approval.requested" => Some(RunEvent::ApprovalRequested {
            approval_id: field(event, "approval_id")?,
            tool_call_id: field(event, "tool_call_id")?,
            tool: field(event, "tool")?,
            action: field(event, "action")?,
            timestamp_ms: event.timestamp_ms,
            proposed_execpolicy_amendment: event.payload.get("proposed_execpolicy_amendment"),
            available_decisions: event.payload.get("available_decisions"),
        }),
        "approval.decided" => Some(RunEvent::ApprovalDecided {
            approval_id: field(event, "approval_id")?,
            decision_id: field(event, "decision_id")?,
            status: field(event, "status")?,
            timestamp_ms: event.timestamp_ms,
        }),
        _ => None,
    }
}
