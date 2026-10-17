//! Assistant text and tool-call content with existing diagnostics.
use super::wire::*;
use crate::*;
pub(super) fn content(choice: &MoonshotChoice) -> (Vec<ContentPart>, bool) {
    let mut content = Vec::new();
    let mut has_tool_calls = false;

    if let Some(text) = &choice.message.content
        && !text.is_empty()
    {
        content.push(ContentPart::Text { text: text.clone() });
    }

    if let Some(tool_calls) = &choice.message.tool_calls {
        has_tool_calls = !tool_calls.is_empty();
        for tc in tool_calls {
            // Log tool call details for debugging (uses role and call_type fields)
            tracing::debug!(
                tool_call_id = %tc.id,
                call_type = %tc.call_type,
                function_name = %tc.function.name,
                "Processing tool call"
            );
            content.push(ContentPart::ToolCall {
                id: tc.id.clone(),
                name: tc.function.name.clone(),
                arguments: tc.function.arguments.clone(),
                thought_signature: None,
            });
        }
    }

    (content, has_tool_calls)
}
