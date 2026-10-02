//! Select raw message payloads for clean copying, independently of rendering.

use super::message::{ChatMessage, MessageType};

#[derive(Clone, Copy)]
pub(crate) enum Target {
    Reply,
    Tool,
    Error,
}

pub(crate) fn target(prompt: &str) -> Option<Target> {
    let mut words = prompt.split_whitespace();
    if words.next()? != "/copy" {
        return None;
    }
    let target = match words.next() {
        None | Some("reply") => Target::Reply,
        Some("tool") => Target::Tool,
        Some("error") => Target::Error,
        Some(_) => return None,
    };
    words.next().is_none().then_some(target)
}

pub(crate) fn latest(messages: &[ChatMessage], target: Target) -> Option<&str> {
    messages.iter().rev().find_map(|message| {
        let text = match (&message.message_type, target) {
            (MessageType::Assistant, Target::Reply) => &message.content,
            (MessageType::ToolResult { output, .. }, Target::Tool)
            | (
                MessageType::ToolResult {
                    output,
                    success: false,
                    ..
                },
                Target::Error,
            ) => output,
            (MessageType::Error, Target::Error) => &message.content,
            _ => return None,
        };
        (!text.trim().is_empty()).then_some(text.as_str())
    })
}

#[cfg(test)]
#[path = "copy_tests.rs"]
mod tests;
