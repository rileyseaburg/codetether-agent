//! approx bytes.
use super::*;

pub(super) fn approx_bytes(msg: &Message) -> usize {
    msg.content
        .iter()
        .map(|p| match p {
            ContentPart::Text { text } => text.len(),
            ContentPart::ToolResult { content, .. } => content.len(),
            ContentPart::ToolCall {
                arguments, name, ..
            } => arguments.len() + name.len(),
            ContentPart::Thinking { .. } => 0,
            ContentPart::Image { .. } | ContentPart::File { .. } => 1024,
        })
        .sum()
}
