//! Response tool-call conversion and diagnostics.
use crate::*;
pub(super) fn append(
    choice: &OpenRouterChoice,
    content: &mut Vec<ContentPart>,
    has_tool_calls: &mut bool,
) {
    // Add tool calls if present
    if let Some(tool_calls) = &choice.message.tool_calls {
        *has_tool_calls = !tool_calls.is_empty();
        for tc in tool_calls {
            // Log tool call details (uses call_type and index fields)
            tracing::debug!(
                tool_call_id = %tc.id,
                call_type = %tc.call_type,
                index = ?tc.index,
                function_name = %tc.function.name,
                "Processing OpenRouter tool call"
            );
            content.push(ContentPart::ToolCall {
                id: tc.id.clone(),
                name: tc.function.name.clone(),
                arguments: tc.function.arguments.clone(),
                thought_signature: None,
            });
        }
    }
}
