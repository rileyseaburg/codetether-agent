//! Application response content filtering and reasoning diagnostics.
use crate::{ContentPart, response_wire::ZaiChoice};
use serde_json::Value;
pub(super) fn convert(choice: &ZaiChoice) -> (Vec<ContentPart>, bool) {
    // Log thinking/reasoning content if present
    if let Some(ref reasoning) = choice.message.reasoning_content
        && !reasoning.is_empty()
    {
        tracing::info!(
            reasoning_len = reasoning.len(),
            "Z.AI reasoning content received"
        );
    }

    let mut content = Vec::new();
    let mut has_tool_calls = false;

    // Emit thinking content as a Thinking part
    if let Some(ref reasoning) = choice.message.reasoning_content
        && !reasoning.is_empty()
    {
        content.push(ContentPart::Thinking {
            text: reasoning.clone(),
            signature: None,
        });
    }

    if let Some(text) = &choice.message.content
        && !text.is_empty()
    {
        content.push(ContentPart::Text { text: text.clone() });
    }

    if let Some(tool_calls) = &choice.message.tool_calls {
        has_tool_calls = !tool_calls.is_empty();
        for tc in tool_calls {
            // Z.AI returns arguments as an object; serialize to string for our ContentPart
            let arguments = match &tc.function.arguments {
                Value::String(s) => s.clone(),
                other => serde_json::to_string(other).unwrap_or_default(),
            };
            content.push(ContentPart::ToolCall {
                id: tc.id.clone(),
                name: tc.function.name.clone(),
                arguments,
                thought_signature: None,
            });
        }
    }

    (content, has_tool_calls)
}
