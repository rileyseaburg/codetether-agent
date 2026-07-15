//! Interpretation of session events emitted by a sub-agent turn.

use super::state::EventLoopState;
use crate::session::SessionEvent;
use serde_json::json;

/// Apply one channel result to the event-loop accumulator.
///
/// # Arguments
///
/// * `result` - The next event, or `None` when the sender has closed.
/// * `state` - Accumulator updated from the received event.
///
/// # Examples
///
/// ```ignore
/// recv_result(Some(SessionEvent::Done), &mut state);
/// ```
pub(super) fn recv_result(result: Option<SessionEvent>, state: &mut EventLoopState) {
    match result {
        Some(event) => apply(event, state),
        None => state.done = true,
    }
}

fn apply(event: SessionEvent, state: &mut EventLoopState) {
    match event {
        SessionEvent::TextComplete(text) => state.response.push_str(&text),
        SessionEvent::ThinkingComplete(text) => state.thinking.push_str(&text),
        SessionEvent::ToolCallComplete {
            name,
            output,
            success,
            ..
        } => state.tools.push(json!({
            "tool": name,
            "success": success,
            "output_preview": crate::tool::agent::text::truncate_preview(&output, 200),
        })),
        SessionEvent::Error(error) => {
            state.response.push_str(&format!("\n[Error: {error}]"));
            state.error = Some(error);
        }
        SessionEvent::ApprovalRequest(request) => super::approve::auto_approve(&request),
        SessionEvent::Done => state.done = true,
        _ => {}
    }
}
