//! Tool-event reduction for an in-flight agent trace.

use super::{LiveTraceEntry, LiveTraceSnapshot};
use crate::TraceEvent;

/// Reduce a tool lifecycle event into the active trace.
pub(super) fn apply(trace: &mut LiveTraceSnapshot, event: TraceEvent<'_>) {
    match event {
        TraceEvent::ToolCallStart {
            name, arguments, ..
        } => {
            trace.entries.push(LiveTraceEntry::ToolCall {
                name: name.to_string(),
                arguments: arguments.to_string(),
            });
            trace.activity = Some(format!("Running {name}…"));
        }
        TraceEvent::ToolHeartbeat {
            name, elapsed_secs, ..
        } => {
            trace.activity = Some(format!("Running {name} · {elapsed_secs}s"));
        }
        TraceEvent::ToolOutputChunk { name, .. } => {
            trace.activity = Some(format!("Streaming output from {name}…"));
        }
        TraceEvent::ToolCallComplete {
            name,
            output,
            success,
            ..
        } => {
            trace.entries.push(LiveTraceEntry::ToolResult {
                name: name.to_string(),
                output: codetether_child_events::truncate_preview(output, 4_000),
                success,
            });
            trace.activity = None;
        }
        _ => {}
    }
}
