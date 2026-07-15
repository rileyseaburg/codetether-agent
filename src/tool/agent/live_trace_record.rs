//! Session-event reduction into a compact live agent trace.

use super::{LiveTraceEntry, LiveTraceSnapshot};
use crate::session::SessionEvent;

/// Reduce a non-tool session event into the active trace.
pub(super) fn apply(trace: &mut LiveTraceSnapshot, event: &SessionEvent) {
    match event {
        SessionEvent::Thinking => trace.activity = Some("Thinking…".into()),
        SessionEvent::TextChunk(text) => {
            trace.streaming_text = Some(text.clone());
            trace.activity = Some("Writing response…".into());
        }
        SessionEvent::TextComplete(text) => complete_text(trace, text),
        SessionEvent::ToolCallStart { .. }
        | SessionEvent::ToolHeartbeat { .. }
        | SessionEvent::ToolOutputChunk { .. }
        | SessionEvent::ToolCallComplete { .. } => super::tool::apply(trace, event),
        SessionEvent::ApprovalRequest(_) => trace.activity = Some("Waiting for approval…".into()),
        SessionEvent::Error(error) => {
            trace.entries.push(LiveTraceEntry::Error(error.clone()));
            trace.activity = None;
        }
        SessionEvent::Done => trace.activity = None,
        _ => {}
    }
}

fn complete_text(trace: &mut LiveTraceSnapshot, text: &str) {
    trace.streaming_text = None;
    trace
        .entries
        .push(LiveTraceEntry::Assistant(text.to_string()));
    trace.activity = None;
}
