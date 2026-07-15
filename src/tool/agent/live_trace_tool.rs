//! Tool-event reduction for an in-flight agent trace.

use super::{LiveTraceEntry, LiveTraceSnapshot};
use crate::session::SessionEvent;

/// Reduce a tool lifecycle event into the active trace.
pub(super) fn apply(trace: &mut LiveTraceSnapshot, event: &SessionEvent) {
    match event {
        SessionEvent::ToolCallStart {
            name, arguments, ..
        } => {
            trace.entries.push(LiveTraceEntry::ToolCall {
                name: name.clone(),
                arguments: arguments.clone(),
            });
            trace.activity = Some(format!("Running {name}…"));
        }
        SessionEvent::ToolHeartbeat {
            name, elapsed_secs, ..
        } => {
            trace.activity = Some(format!("Running {name} · {elapsed_secs}s"));
        }
        SessionEvent::ToolOutputChunk { name, .. } => {
            trace.activity = Some(format!("Streaming output from {name}…"));
        }
        SessionEvent::ToolCallComplete {
            name,
            output,
            success,
            ..
        } => {
            trace.entries.push(LiveTraceEntry::ToolResult {
                name: name.clone(),
                output: crate::tool::agent::text::truncate_preview(output, 4_000),
                success: *success,
            });
            trace.activity = None;
        }
        _ => {}
    }
}
