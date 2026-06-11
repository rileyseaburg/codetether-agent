//! JSONL conversion for tool thread events.

use crate::session::thread_store::ThreadEvent;

use super::{event::RunEvent, thread::field};

pub(super) fn to_run_event(event: &ThreadEvent) -> Option<RunEvent<'_>> {
    match event.kind.as_str() {
        "tool.started" => Some(RunEvent::ToolCallStarted {
            item_id: field(event, "item_id")?,
            tool_call_id: field(event, "tool_call_id")?,
            timestamp_ms: event.timestamp_ms,
            name: field(event, "name"),
            arguments: field(event, "arguments"),
        }),
        "tool.completed" => Some(RunEvent::ToolCallCompleted {
            item_id: field(event, "item_id")?,
            tool_call_id: field(event, "tool_call_id")?,
            timestamp_ms: event.timestamp_ms,
            name: field(event, "name"),
            success: event.payload.get("success").and_then(|v| v.as_bool()),
            duration_ms: event.payload.get("duration_ms").and_then(|v| v.as_u64()),
            output: field(event, "output"),
        }),
        "tool.metadata" => Some(RunEvent::ToolCallMetadata {
            item_id: field(event, "item_id")?,
            tool_call_id: field(event, "tool_call_id")?,
            timestamp_ms: event.timestamp_ms,
            metadata: event.payload.get("metadata")?,
        }),
        _ => None,
    }
}
