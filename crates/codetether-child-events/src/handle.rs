//! Interpretation of session events emitted by a sub-agent turn.

use super::state::EventLoopState;
use crate::{Event, Host};
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
/// recv_result(Some(Event::Done), &mut state);
/// ```
pub(super) fn recv_result<H: Host>(result: Option<Event<H::Approval>>, state: &mut EventLoopState) {
    match result {
        Some(event) => apply::<H>(event, state),
        None => state.done = true,
    }
}

fn apply<H: Host>(event: Event<H::Approval>, state: &mut EventLoopState) {
    match event {
        Event::TextComplete(text) => state.response.push_str(&text),
        Event::ThinkingComplete(text) => state.thinking.push_str(&text),
        Event::ToolCallComplete {
            name,
            output,
            success,
            ..
        } => state.tools.push(json!({
            "tool": name,
            "success": success,
            "output_preview": super::text::truncate_preview(&output, 200),
        })),
        Event::Error(error) => {
            state.response.push_str(&format!("\n[Error: {error}]"));
            state.error = Some(error);
        }
        Event::ApprovalRequest(request) => H::approve(&request),
        Event::Done => state.done = true,
        Event::Ignored => {}
    }
}
