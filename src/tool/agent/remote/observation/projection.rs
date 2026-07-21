//! Conversion from A2A activity metadata to the existing TUI live trace.

use super::projection_value::{display, text};
use super::types::RemoteTurn;
use crate::tool::agent::event_loop::live_trace::{LiveTraceEntry, LiveTraceSnapshot};
use serde_json::Value;

pub(super) fn trace(turn: &RemoteTurn) -> Option<LiveTraceSnapshot> {
    turn.is_processing.then(|| {
        let mut trace = LiveTraceSnapshot {
            prompt: turn.prompt.clone(),
            activity: Some("Waiting for A2A peer activity…".into()),
            ..Default::default()
        };
        for event in &turn.activity {
            apply(&mut trace, event);
        }
        trace
    })
}

fn apply(trace: &mut LiveTraceSnapshot, event: &Value) {
    match event.get("type").and_then(Value::as_str) {
        Some("thinking") => trace.activity = Some("Thinking…".into()),
        Some("thinking_complete") => trace.activity = Some("Reasoning complete".into()),
        Some("text_chunk" | "text_complete") => {
            trace.streaming_text = text(event, "text").map(ToString::to_string)
        }
        Some("tool_call_start") => trace.entries.push(LiveTraceEntry::ToolCall {
            name: text(event, "name").unwrap_or("tool").to_string(),
            arguments: display(&event["arguments"]),
        }),
        Some("tool_call_complete") => trace.entries.push(LiveTraceEntry::ToolResult {
            name: text(event, "name").unwrap_or("tool").to_string(),
            output: display(&event["output"]),
            success: event
                .get("success")
                .and_then(Value::as_bool)
                .unwrap_or(false),
        }),
        Some("error") => trace.entries.push(LiveTraceEntry::Error(
            text(event, "error")
                .unwrap_or("Remote agent failed")
                .to_string(),
        )),
        Some("done") => trace.activity = Some("Finishing response…".into()),
        _ => {}
    }
}
