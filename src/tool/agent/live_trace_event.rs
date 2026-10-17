//! Borrow only the native event fields needed by live-trace reduction.
use crate::session::SessionEvent;
use codetether_agent_trace::TraceEvent;
pub(super) fn project(event: &SessionEvent) -> TraceEvent<'_> {
    match event {
        SessionEvent::Thinking => TraceEvent::Thinking,
        SessionEvent::TextChunk(text) => TraceEvent::TextChunk(text),
        SessionEvent::TextComplete(text) => TraceEvent::TextComplete(text),
        SessionEvent::ToolCallStart {
            name, arguments, ..
        } => TraceEvent::ToolCallStart { name, arguments },
        SessionEvent::ToolHeartbeat {
            name, elapsed_secs, ..
        } => TraceEvent::ToolHeartbeat {
            name,
            elapsed_secs: *elapsed_secs,
        },
        SessionEvent::ToolOutputChunk { name, .. } => TraceEvent::ToolOutputChunk { name },
        SessionEvent::ToolCallComplete {
            name,
            output,
            success,
            ..
        } => TraceEvent::ToolCallComplete {
            name,
            output,
            success: *success,
        },
        SessionEvent::ApprovalRequest(_) => TraceEvent::ApprovalRequest,
        SessionEvent::Error(error) => TraceEvent::Error(error),
        SessionEvent::Done => TraceEvent::Done,
        _ => TraceEvent::Ignored,
    }
}
