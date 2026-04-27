//! Message extraction helpers for agent execution.
//!
//! This module converts provider messages into tool-call lists and final text
//! responses used by the main execution loop.
//!
//! # Examples
//!
//! ```ignore
//! let text = response_text(&message);
//! ```

use crate::agent::Agent;
use crate::provider::{ContentPart, Message};

/// Internal representation of a pending provider tool call.
///
/// # Examples
///
/// ```ignore
/// let call: PendingToolCall = ("id".into(), "bash".into(), "{}".into());
/// ```
pub(super) type PendingToolCall = (String, String, String);

impl Agent {
    pub(super) fn build_messages(
        &self,
        system_prompt: String,
        derived: Vec<Message>,
    ) -> Vec<Message> {
        let mut messages = vec![Message {
            role: crate::provider::Role::System,
            content: vec![ContentPart::Text {
                text: system_prompt,
            }],
        }];
        messages.extend(derived);
        messages
    }
}

/// Extracts all tool calls from a provider message.
///
/// # Examples
///
/// ```ignore
/// let calls = collect_tool_calls(&message);
/// ```
pub(super) fn collect_tool_calls(message: &Message) -> Vec<PendingToolCall> {
    message
        .content
        .iter()
        .filter_map(extract_tool_call)
        .collect()
}

/// Extracts the concatenated text content from a provider message.
///
/// # Examples
///
/// ```ignore
/// let text = response_text(&message);
/// ```
pub(super) fn response_text(message: &Message) -> String {
    message
        .content
        .iter()
        .filter_map(extract_text)
        .collect::<Vec<_>>()
        .join("\n")
}

fn extract_tool_call(part: &ContentPart) -> Option<PendingToolCall> {
    match part {
        ContentPart::ToolCall {
            id,
            name,
            arguments,
            ..
        } => Some((id.clone(), name.clone(), arguments.clone())),
        _ => None,
    }
}

fn extract_text(part: &ContentPart) -> Option<String> {
    match part {
        ContentPart::Text { text } => Some(text.clone()),
        _ => None,
    }
}
