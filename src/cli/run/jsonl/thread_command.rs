//! JSONL conversion for command thread events.

use crate::session::thread_store::ThreadEvent;

use super::{event::RunEvent, thread::field};

pub(super) fn to_run_event(event: &ThreadEvent) -> Option<RunEvent<'_>> {
    match event.kind.as_str() {
        "command.started" => Some(RunEvent::CommandStarted {
            item_id: field(event, "item_id")?,
            tool_call_id: field(event, "tool_call_id")?,
            timestamp_ms: event.timestamp_ms,
            command: field(event, "command"),
            cwd: field(event, "cwd"),
        }),
        "command.completed" => Some(RunEvent::CommandCompleted {
            item_id: field(event, "item_id")?,
            tool_call_id: field(event, "tool_call_id")?,
            timestamp_ms: event.timestamp_ms,
            success: event.payload.get("success")?.as_bool()?,
            duration_ms: event.payload.get("duration_ms")?.as_u64()?,
        }),
        _ => None,
    }
}
