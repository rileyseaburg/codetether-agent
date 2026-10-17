//! Tool-call precedence and provider finish reasons.
use super::wire::*;
use crate::*;
pub(super) fn finish(choice: &MoonshotChoice, has_tool_calls: bool) -> FinishReason {
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
