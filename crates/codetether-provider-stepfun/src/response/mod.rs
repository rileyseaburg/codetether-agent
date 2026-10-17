//! Response selection, metadata, content, and usage.
use crate::*;
mod content;
mod finish;
mod usage;
pub(crate) fn parse(chat_response: ChatResponse) -> Result<CompletionResponse> {
    // Log response metadata for debugging
    tracing::debug!(
        response_id = %chat_response.id,
        "Received StepFun response"
    );

    let choice = chat_response
        .choices
        .first()
        .ok_or_else(|| anyhow::anyhow!("No choices in response"))?;

    // Log choice index and role for debugging
    tracing::debug!(
        choice_index = choice.index,
        message_role = %choice.message.role,
        "Processing StepFun choice"
    );

    // Log usage for tracing
    tracing::info!(
        prompt_tokens = chat_response.usage.as_ref().map(|u| u.prompt_tokens).unwrap_or(0),
        completion_tokens = chat_response.usage.as_ref().map(|u| u.completion_tokens).unwrap_or(0),
        finish_reason = ?choice.finish_reason,
        "StepFun completion received"
    );

    let (content, has_tool_calls) = content::convert(choice);
    let finish_reason = finish::finish(choice, has_tool_calls);
    Ok(CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage: usage::convert(&chat_response),
        finish_reason,
    })
}
