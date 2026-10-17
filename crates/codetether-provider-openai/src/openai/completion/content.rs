//! content for the OpenAI-compatible transport.

use super::*;

pub(super) fn completion_content(
    choice: &async_openai::types::chat::ChatChoice,
) -> (Vec<ContentPart>, bool) {
    let mut content = Vec::new();
    let mut has_tool_calls = false;

    if let Some(text) = &choice.message.content {
        content.push(ContentPart::Text { text: text.clone() });
    }
    if let Some(tool_calls) = &choice.message.tool_calls {
        has_tool_calls = !tool_calls.is_empty();
        for tc in tool_calls {
            if let ChatCompletionMessageToolCalls::Function(func_call) = tc {
                content.push(ContentPart::ToolCall {
                    id: func_call.id.clone(),
                    name: func_call.function.name.clone(),
                    arguments: func_call.function.arguments.clone(),
                    thought_signature: None,
                });
            }
        }
    }

    (content, has_tool_calls)
}
