//! Session-event reduction into a compact live agent trace.

use super::{LiveTraceEntry, LiveTraceSnapshot};
use crate::TraceEvent;

/// Reduce a non-tool session event into the active trace.
pub(super) fn apply(trace: &mut LiveTraceSnapshot, event: TraceEvent<'_>) {
    match event {
        TraceEvent::Thinking => trace.activity = Some("Thinking…".into()),
        TraceEvent::TextChunk(text) => {
            trace.streaming_text = Some(text.to_string());
            trace.activity = Some("Writing response…".into());
        }
        TraceEvent::TextComplete(text) => complete_text(trace, text),
        TraceEvent::ToolCallStart { .. }
        | TraceEvent::ToolHeartbeat { .. }
        | TraceEvent::ToolOutputChunk { .. }
        | TraceEvent::ToolCallComplete { .. } => super::tool::apply(trace, event),
        TraceEvent::ApprovalRequest => trace.activity = Some("Waiting for approval…".into()),
        TraceEvent::Error(error) => {
            trace.entries.push(LiveTraceEntry::Error(error.to_string()));
            trace.activity = None;
        }
        TraceEvent::Done => trace.activity = None,
        TraceEvent::Ignored => {}
    }
}

fn complete_text(trace: &mut LiveTraceSnapshot, text: &str) {
    trace.streaming_text = None;
    trace
        .entries
        .push(LiveTraceEntry::Assistant(text.to_string()));
    trace.activity = None;
}
