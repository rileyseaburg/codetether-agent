//! Application finish reason precedence.
use crate::{FinishReason, response_wire::ZaiChoice};
pub(super) fn finish(choice: &ZaiChoice, has_tool_calls: bool) -> FinishReason {
    let finish_reason = if has_tool_calls {
        FinishReason::ToolCalls
    } else {
        match choice.finish_reason.as_deref() {
            Some("stop") => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            Some("tool_calls") => FinishReason::ToolCalls,
            Some("sensitive") => FinishReason::ContentFilter,
            _ => FinishReason::Stop,
        }
    };

    finish_reason
}
