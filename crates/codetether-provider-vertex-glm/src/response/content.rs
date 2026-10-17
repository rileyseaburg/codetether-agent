//! Text and tool-call response content.
use crate::*;
pub(super) fn content(choice: &Choice) -> (Vec<ContentPart>, bool) {
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
