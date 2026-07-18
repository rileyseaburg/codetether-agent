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

use crate::provider::{ContentPart, Message};
use crate::session::Session;

/// Compose the system prompt by appending the session's goal-governance
/// block (if any) to the agent's base persona prompt.
///
/// Reads `<sessions_dir>/<session-id>.tasks.jsonl` synchronously; the
/// file is small (a few KB at most) and this is used from the sync
/// `complete_with_context` prompt-composition path without a tokio handle.
pub(super) fn compose_system_prompt(base: &str, session: &Session) -> String {
    crate::session::tasks::runtime::compose(base, &session.id)
}

/// Extracts the canonical tool-call content parts from a provider message.
///
/// Returns owned [`ContentPart::ToolCall`] values (preserving
/// `thought_signature`) rather than flattening into positional tuples.
///
/// # Examples
///
/// ```ignore
/// let calls = collect_tool_calls(&message);
/// ```
pub(super) fn collect_tool_calls(message: &Message) -> Vec<ContentPart> {
    message
        .content
        .iter()
        .filter(|p| p.as_tool_call().is_some())
        .cloned()
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

fn extract_text(part: &ContentPart) -> Option<String> {
    match part {
        ContentPart::Text { text } => Some(text.clone()),
        _ => None,
    }
}
