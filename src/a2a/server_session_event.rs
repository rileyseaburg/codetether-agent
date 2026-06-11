//! A2A JSON payloads for selected session events.

use crate::session::SessionEvent;
use serde_json::{Value, json};

pub(super) fn data(event: &SessionEvent) -> Value {
    match event {
        SessionEvent::Thinking => json!({ "type": "thinking" }),
        SessionEvent::ToolCallStart {
            tool_call_id,
            name,
            arguments,
        } => json!({
            "type": "tool_call_start",
            "tool_call_id": tool_call_id,
            "name": name,
            "arguments": arguments
        }),
        SessionEvent::ToolCallComplete {
            tool_call_id,
            name,
            output,
            success,
            duration_ms,
        } => json!({
            "type": "tool_call_complete",
            "tool_call_id": tool_call_id,
            "name": name,
            "output": output.chars().take(500).collect::<String>(),
            "success": success,
            "duration_ms": duration_ms
        }),
        _ => json!({ "type": "ignored" }),
    }
}
