//! A2A JSON payloads for selected session events.

use crate::session::SessionEvent;
use codetether_a2a_core::observer_payloads as payload;
use serde_json::Value;

pub(super) fn data(event: &SessionEvent) -> Value {
    match event {
        SessionEvent::Thinking => payload::thinking(),
        SessionEvent::ToolCallStart {
            tool_call_id,
            name,
            arguments,
        } => payload::tool_call_start(tool_call_id, name, arguments),
        SessionEvent::ToolCallComplete {
            tool_call_id,
            name,
            output,
            success,
            duration_ms,
        } => payload::tool_call_complete(tool_call_id, name, output, *success, *duration_ms),
        SessionEvent::ToolHeartbeat {
            tool_call_id,
            name,
            elapsed_secs,
        } => payload::tool_heartbeat(tool_call_id, name, *elapsed_secs),
        SessionEvent::ToolOutputChunk {
            tool_call_id,
            name,
            stream,
            chunk,
        } => payload::tool_output_chunk(tool_call_id, name, stream, chunk),
        _ => payload::ignored(),
    }
}
