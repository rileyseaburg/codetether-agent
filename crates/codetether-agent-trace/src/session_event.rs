//! Borrow only the native event fields needed by live-trace reduction.
use crate::TraceEvent;
use codetether_session_events::SessionEvent;
/// Project and record one session event in the child's active trace.
/// # Arguments
/// * `name` - Child name used by the trace store.
/// * `event` - Session event to project without moving its payload.
/// # Returns
/// Updates the same live trace as [`crate::observe`].
/// # Examples
/// ```
/// codetether_agent_trace::observe_session("child", &codetether_session_events::SessionEvent::<()>::Thinking);
/// ```
pub fn observe_session<S>(name: &str, event: &SessionEvent<S>) {
    crate::observe(name, project(event));
}
/// Borrow the trace-relevant fields of a streaming session event.
/// # Arguments
/// * `event` - Session event whose payload remains owned by the caller.
/// # Returns
/// A borrowed projection, or `Ignored` for events without trace effects.
/// # Examples
/// ```
/// let event = codetether_session_events::SessionEvent::<()>::Done;
/// assert!(matches!(codetether_agent_trace::project(&event), codetether_agent_trace::TraceEvent::Done));
/// ```
pub fn project<S>(event: &SessionEvent<S>) -> TraceEvent<'_> {
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
