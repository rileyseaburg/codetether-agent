//! Parsing and finish-reason selection for buffered completions.
use crate::{CompletionResponse, ContentPart, FinishReason, Message, Role};
use anyhow::{Context, Result};

pub(crate) fn parse(text: &str) -> Result<CompletionResponse> {
    let response: crate::response_wire::ZaiResponse =
        serde_json::from_str(text).context("Failed to parse Z.AI response")?;
    let choice = response
        .choices
        .into_iter()
        .next()
        .context("No choices in Z.AI response")?;
    let content = crate::response_content::convert(choice.message);
    let finish_reason = if content
        .iter()
        .any(|part| matches!(part, ContentPart::ToolCall { .. }))
    {
        FinishReason::ToolCalls
    } else {
        match choice.finish_reason.as_deref() {
            Some("length") => FinishReason::Length,
            Some("tool_calls") => FinishReason::ToolCalls,
            Some("sensitive") => FinishReason::ContentFilter,
            _ => FinishReason::Stop,
        }
    };
    Ok(CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage: response
            .usage
            .as_ref()
            .map(crate::usage::convert)
            .unwrap_or_default(),
        finish_reason,
    })
}
