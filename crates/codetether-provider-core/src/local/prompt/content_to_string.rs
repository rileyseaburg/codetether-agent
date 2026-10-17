//! Local-provider content to string.
use super::*;
impl LocalCudaPrompt {
    /// Join supported text, tool-result, and thinking parts for a local prompt.
    pub fn content_to_string(parts: &[ContentPart]) -> String {
        parts
            .iter()
            .map(|part| match part {
                ContentPart::Text { text } => text.clone(),
                ContentPart::ToolResult { content, .. } => content.clone(),
                ContentPart::Thinking { text, .. } => text.clone(),
                _ => String::new(),
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
