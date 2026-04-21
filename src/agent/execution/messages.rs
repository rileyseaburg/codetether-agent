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
use crate::session::Session;

/// Internal representation of a pending provider tool call.
///
/// # Examples
///
/// ```ignore
/// let call: PendingToolCall = ("id".into(), "bash".into(), "{}".into());
/// ```
pub(super) type PendingToolCall = (String, String, String);

impl Agent {
    pub(super) fn build_messages(&self, session: &Session) -> Vec<Message> {
        let mut messages = vec![Message {
            role: crate::provider::Role::System,
            content: vec![ContentPart::Text {
                text: compose_system_prompt(&self.system_prompt, session),
            }],
        }];
        messages.extend(session.messages.clone());
        messages
    }
}

/// Compose the system prompt by appending the session's goal-governance
/// block (if any) to the agent's base persona prompt.
///
/// Reads `<sessions_dir>/<session-id>.tasks.jsonl` synchronously; the
/// file is small (a few KB at most) and we want this to run inside the
/// sync `build_messages` path without a tokio handle.
fn compose_system_prompt(base: &str, session: &Session) -> String {
    let log = match crate::session::tasks::TaskLog::for_session(&session.id) {
        Ok(l) => l,
        Err(_) => return base.to_string(),
    };
    let events = log.read_all_blocking().unwrap_or_default();
    let state = crate::session::tasks::TaskState::from_log(&events);
    match crate::session::tasks::governance_block(&state) {
        Some(block) => format!("{base}\n\n{block}"),
        None => base.to_string(),
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
