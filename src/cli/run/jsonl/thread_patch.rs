//! JSONL conversion for patch thread events.

use crate::session::thread_store::ThreadEvent;

use super::{event::RunEvent, thread::field};

pub(super) fn to_run_event(event: &ThreadEvent) -> Option<RunEvent<'_>> {
    match event.kind.as_str() {
        "patch.started" => Some(RunEvent::PatchStarted {
            patch_id: field(event, "patch_id")?,
            timestamp_ms: event.timestamp_ms,
        }),
        "patch.completed" => Some(RunEvent::PatchCompleted {
            patch_id: field(event, "patch_id")?,
            timestamp_ms: event.timestamp_ms,
            metadata: &event.payload,
        }),
        "patch.approval_required" => Some(RunEvent::PatchApprovalRequired {
            approval_id: field(event, "approval_id")?,
            patch_id: field(event, "patch_id")?,
            timestamp_ms: event.timestamp_ms,
        }),
        _ => None,
    }
}
