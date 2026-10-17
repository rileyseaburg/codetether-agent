//! Completion reason with tool-call precedence.
use crate::*;
pub(super) fn finish(choice: &Glm5Choice, has_tool_calls: bool) -> FinishReason {
    let finish_reason = if has_tool_calls {
        FinishReason::ToolCalls
    } else {
        match choice.finish_reason.as_deref() {
            Some("stop") => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            Some("tool_calls") => FinishReason::ToolCalls,
            _ => FinishReason::Stop,
        }
    };
    finish_reason
}
