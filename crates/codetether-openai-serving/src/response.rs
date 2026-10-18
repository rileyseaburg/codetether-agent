//! Nonstreaming OpenAI completion envelope and usage fields.
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use codetether_openai_adapter::{convert_finish_reason, convert_response_message};
use codetether_openai_wire::{
    OpenAiChatCompletionChoice, OpenAiChatCompletionMessage, OpenAiChatCompletionResponse,
    OpenAiUsage,
};
use codetether_provider_core::CompletionResponse;
pub(crate) fn build(
    completion: CompletionResponse,
    chat_id: String,
    now: i64,
    openai_model: String,
) -> Response {
    let (content, tool_calls) = convert_response_message(&completion.message);

    Json(OpenAiChatCompletionResponse {
        id: chat_id,
        object: "chat.completion".to_string(),
        created: now,
        model: openai_model,
        choices: vec![OpenAiChatCompletionChoice {
            index: 0,
            message: OpenAiChatCompletionMessage {
                role: "assistant".to_string(),
                content,
                tool_calls,
            },
            finish_reason: convert_finish_reason(completion.finish_reason).to_string(),
        }],
        usage: OpenAiUsage {
            prompt_tokens: completion.usage.prompt_tokens,
            completion_tokens: completion.usage.completion_tokens,
            total_tokens: completion.usage.total_tokens,
        },
    })
    .into_response()
}
