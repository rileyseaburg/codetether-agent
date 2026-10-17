//! Response content and thought-signature extraction.
use super::wire::*;
use crate::*;
pub(super) fn convert(message: ChoiceMessage) -> (Vec<ContentPart>, bool) {
    let mut content_parts = Vec::new();
    let mut has_tool_calls = false;

    if let Some(text) = message.content
        && !text.is_empty()
    {
        content_parts.push(ContentPart::Text { text });
    }

    if let Some(tool_calls) = message.tool_calls {
        has_tool_calls = !tool_calls.is_empty();
        for tc in tool_calls {
            // Extract thought signature from extra_content.google.thought_signature
            let thought_signature = tc
                .extra_content
                .as_ref()
                .and_then(|ec| ec.google.as_ref())
                .and_then(|g| g.thought_signature.clone());

            content_parts.push(ContentPart::ToolCall {
                id: tc.id,
                name: tc.function.name,
                arguments: tc.function.arguments,
                thought_signature,
            });
        }
    }

    (content_parts, has_tool_calls)
}
