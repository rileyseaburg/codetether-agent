//! Finish-reason mapping for Copilot choices.

use super::types::CopilotChoice;
use crate::provider::FinishReason;

pub(super) fn reason(choices: &[CopilotChoice], has_tool_calls: bool) -> FinishReason {
    if has_tool_calls || has_reason(choices, "tool_calls") {
        FinishReason::ToolCalls
    } else if has_reason(choices, "length") {
        FinishReason::Length
    } else if has_reason(choices, "content_filter") {
        FinishReason::ContentFilter
    } else {
        FinishReason::Stop
    }
}

fn has_reason(choices: &[CopilotChoice], needle: &str) -> bool {
    choices
        .iter()
        .any(|choice| choice.finish_reason.as_deref() == Some(needle))
}
