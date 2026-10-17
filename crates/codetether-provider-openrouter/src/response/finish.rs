//! Finish reason precedence.
use crate::*;
pub(super) fn finish(choice: &OpenRouterChoice, has_tool_calls: bool) -> FinishReason {
    // Determine finish reason
    let finish_reason = if has_tool_calls {
        FinishReason::ToolCalls
    } else {
        match choice.finish_reason.as_deref() {
            Some("stop") => FinishReason::Stop,
            Some("length") => FinishReason::Length,
            Some("tool_calls") => FinishReason::ToolCalls,
            Some("content_filter") => FinishReason::ContentFilter,
            _ => FinishReason::Stop,
        }
    };

    finish_reason
}
