use super::constants::CHARS_PER_TOKEN;
use crate::provider::{ContentPart, Message};

pub fn estimate_tokens(text: &str) -> usize {
    (text.len() as f64 / CHARS_PER_TOKEN).ceil() as usize
}

pub fn estimate_message_tokens(message: &Message) -> usize {
    let mut tokens = 4;
    for part in &message.content {
        tokens += match part {
            ContentPart::Text { text } => estimate_tokens(text),
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => estimate_tokens(id) + estimate_tokens(name) + estimate_tokens(arguments) + 10,
            ContentPart::ToolResult {
                tool_call_id,
                content,
            } => estimate_tokens(tool_call_id) + estimate_tokens(content) + 6,
            ContentPart::Image { .. } | ContentPart::File { .. } => 2000,
            ContentPart::Thinking { text } => estimate_tokens(text),
        };
    }
    tokens
}

pub fn estimate_total_tokens(messages: &[Message]) -> usize {
    messages.iter().map(estimate_message_tokens).sum()
}
