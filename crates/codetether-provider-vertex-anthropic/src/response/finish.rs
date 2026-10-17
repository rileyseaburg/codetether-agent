//! Stop reason and tool-call precedence.
use crate::*;
pub(super) fn finish(response: &AnthropicResponse, has_tool_calls: bool) -> FinishReason {
    let finish_reason = if has_tool_calls {
        FinishReason::ToolCalls
    } else {
        match response.stop_reason.as_deref() {
            Some("end_turn") | Some("stop") => FinishReason::Stop,
            Some("max_tokens") => FinishReason::Length,
            Some("tool_use") => FinishReason::ToolCalls,
            Some("content_filter") => FinishReason::ContentFilter,
            _ => FinishReason::Stop,
        }
    };

    finish_reason
}
