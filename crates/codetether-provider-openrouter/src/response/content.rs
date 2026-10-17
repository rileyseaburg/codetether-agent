//! Thinking, text, refusal, and tool-call content.
use crate::*;
pub(super) fn content(choice: &OpenRouterChoice) -> (Vec<ContentPart>, bool) {
    let mut content = Vec::new();
    let mut has_tool_calls = false;

    // Surface reasoning as thinking content so a non-streaming turn keeps
    // the same shape as the streaming path, which emits StreamChunk::Thinking.
    if let Some(reasoning) = choice
        .message
        .reasoning
        .as_deref()
        .filter(|s| !s.is_empty())
    {
        content.push(ContentPart::Thinking {
            text: reasoning.to_string(),
            signature: None,
        });
    }

    // Add text content if present
    if let Some(text) = &choice.message.content
        && !text.is_empty()
    {
        content.push(ContentPart::Text { text: text.clone() });
    }

    // Log message role for debugging
    tracing::debug!(message_role = %choice.message.role, "OpenRouter message role");

    // Log refusal if present (model declined to respond)
    if let Some(ref refusal) = choice.message.refusal {
        tracing::warn!(refusal = %refusal, "Model refused to respond");
    }

    super::tools::append(choice, &mut content, &mut has_tool_calls);
    (content, has_tool_calls)
}
