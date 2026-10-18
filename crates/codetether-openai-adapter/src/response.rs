//! OpenAI-compatible response conversion.
use codetether_openai_wire::{OpenAiResponseToolCall, OpenAiResponseToolCallFunction};

/// Project provider text and tool calls into response payloads.
/// # Arguments
/// message is the completed provider response.
/// # Returns
/// Optional joined text and optional tool calls.
/// # Examples
/// ```
/// let message = codetether_provider_core::Message { role: codetether_provider_core::Role::Assistant, content: vec![] };
/// assert_eq!(codetether_openai_adapter::convert_response_message(&message).0, None);
/// ```
pub fn convert_response_message(
    message: &codetether_provider_core::Message,
) -> (Option<String>, Option<Vec<OpenAiResponseToolCall>>) {
    let mut texts = Vec::new();
    let mut tool_calls = Vec::new();

    for part in &message.content {
        match part {
            codetether_provider_core::ContentPart::Text { text } => {
                if !text.is_empty() {
                    texts.push(text.clone());
                }
            }
            codetether_provider_core::ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => {
                tool_calls.push(OpenAiResponseToolCall {
                    id: id.clone(),
                    kind: "function".to_string(),
                    function: OpenAiResponseToolCallFunction {
                        name: name.clone(),
                        arguments: arguments.clone(),
                    },
                });
            }
            _ => {}
        }
    }

    let content = if texts.is_empty() {
        None
    } else {
        Some(texts.join("\n"))
    };
    let tool_calls = if tool_calls.is_empty() {
        None
    } else {
        Some(tool_calls)
    };

    (content, tool_calls)
}
