//! Text, thinking, and tool-use response conversion.
use crate::*;
pub(super) fn content(response: &AnthropicResponse) -> (Vec<ContentPart>, bool) {
    let mut content = Vec::new();
    let mut has_tool_calls = false;

    for part in &response.content {
        match part {
            AnthropicContent::Text { text } => {
                if !text.is_empty() {
                    content.push(ContentPart::Text { text: text.clone() });
                }
            }
            AnthropicContent::Thinking { thinking, text } => {
                let reasoning = thinking
                    .as_deref()
                    .or(text.as_deref())
                    .unwrap_or_default()
                    .trim()
                    .to_string();
                if !reasoning.is_empty() {
                    content.push(ContentPart::Thinking {
                        text: reasoning,
                        signature: None,
                    });
                }
            }
            AnthropicContent::ToolUse { id, name, input } => {
                has_tool_calls = true;
                content.push(ContentPart::ToolCall {
                    id: id.clone(),
                    name: name.clone(),
                    arguments: serde_json::to_string(input).unwrap_or_default(),
                    thought_signature: None,
                });
            }
            AnthropicContent::Unknown => {}
        }
    }

    (content, has_tool_calls)
}
