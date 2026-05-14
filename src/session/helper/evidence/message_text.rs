use crate::provider::{ContentPart, Message};

pub(crate) fn joined(messages: &[Message]) -> String {
    messages
        .iter()
        .flat_map(|msg| msg.content.iter())
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            ContentPart::ToolResult { content, .. } => Some(content.as_str()),
            ContentPart::Thinking { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}
