//! Map native Anthropic content blocks into crate [`ContentPart`]s.

use super::types::AnthropicContent;
use crate::provider::ContentPart;

/// Returns the mapped content parts and whether any tool call was present.
pub(in crate::provider::bedrock) fn map_content(
    parts: Vec<AnthropicContent>,
) -> (Vec<ContentPart>, bool) {
    let mut content = Vec::new();
    let mut has_tool_calls = false;
    for part in parts {
        match part {
            AnthropicContent::Text { text } if !text.is_empty() => {
                content.push(ContentPart::Text { text });
            }
            AnthropicContent::Thinking {
                thinking,
                signature,
            } => {
                if !thinking.is_empty() || signature.is_some() {
                    content.push(ContentPart::Thinking {
                        text: thinking,
                        signature,
                    });
                }
            }
            AnthropicContent::ToolUse { id, name, input } => {
                has_tool_calls = true;
                content.push(ContentPart::ToolCall {
                    id,
                    name,
                    arguments: serde_json::to_string(&input).unwrap_or_default(),
                    thought_signature: None,
                });
            }
            _ => {}
        }
    }
    (content, has_tool_calls)
}
