//! response for the OpenAI-compatible transport.

use super::*;

pub(super) fn completion_response(
    response: async_openai::types::chat::CreateChatCompletionResponse,
) -> Result<CompletionResponse> {
    let choice = response
        .choices
        .first()
        .ok_or_else(|| anyhow::anyhow!("No choices"))?;

    let (content, has_tool_calls) = completion_content(choice);
    let finish_reason = completion_finish(choice, has_tool_calls);
    Ok(CompletionResponse {
        message: Message {
            role: Role::Assistant,
            content,
        },
        usage: Usage {
            prompt_tokens: response
                .usage
                .as_ref()
                .map(|u| u.prompt_tokens as usize)
                .unwrap_or(0),
            completion_tokens: response
                .usage
                .as_ref()
                .map(|u| u.completion_tokens as usize)
                .unwrap_or(0),
            total_tokens: response
                .usage
                .as_ref()
                .map(|u| u.total_tokens as usize)
                .unwrap_or(0),
            ..Default::default()
        },
        finish_reason,
    })
}
