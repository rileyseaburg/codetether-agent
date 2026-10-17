//! finish for the OpenAI-compatible transport.

use super::*;

pub(super) fn completion_finish(
    choice: &async_openai::types::chat::ChatChoice,
    has_tool_calls: bool,
) -> FinishReason {
    if has_tool_calls {
        FinishReason::ToolCalls
    } else {
        match choice.finish_reason {
            Some(OpenAIFinishReason::Stop) => FinishReason::Stop,
            Some(OpenAIFinishReason::Length) => FinishReason::Length,
            Some(OpenAIFinishReason::ToolCalls) => FinishReason::ToolCalls,
            Some(OpenAIFinishReason::ContentFilter) => FinishReason::ContentFilter,
            _ => FinishReason::Stop,
        }
    }
}
