//! Resolve pending background RLM placeholders in derived prompts.

use crate::provider::{ContentPart, Message};

use super::cache;

pub(super) fn messages(messages: &mut [Message]) {
    for message in messages {
        for part in &mut message.content {
            if let ContentPart::ToolResult { content, .. } = part
                && let Some(key) = key(content)
                && let Some(summary) = cache::ready(key)
            {
                *content = summary;
            }
        }
    }
}

fn key(content: &str) -> Option<u64> {
    let header = content.lines().next()?;
    let raw = header
        .split_whitespace()
        .find_map(|part| part.strip_prefix("key="))?;
    u64::from_str_radix(raw, 16).ok()
}
