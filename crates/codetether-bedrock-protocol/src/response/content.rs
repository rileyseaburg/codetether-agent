//! Ordered Converse content decoding, retaining signed reasoning.
use super::*;
pub(super) fn content(response: &ConverseResponse) -> (Vec<ContentPart>, bool) {
    let mut content = Vec::new();
    let mut has_tool_calls = false;

    for part in &response.output.message.content {
        match part {
            ConverseContent::ReasoningContent { reasoning_content } => {
                if let Some(part) = reasoning_content.to_content_part() {
                    content.push(part);
                }
            }
            ConverseContent::Text { text } => {
                if !text.is_empty() {
                    content.push(ContentPart::Text { text: text.clone() });
                }
            }
            ConverseContent::ToolUse { tool_use } => {
                has_tool_calls = true;
                content.push(ContentPart::ToolCall {
                    id: tool_use.tool_use_id.clone(),
                    name: tool_use.name.clone(),
                    arguments: serde_json::to_string(&tool_use.input).unwrap_or_default(),
                    thought_signature: None,
                });
            }
        }
    }
    (content, has_tool_calls)
}
