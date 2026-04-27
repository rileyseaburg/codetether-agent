use serde::{Deserialize, Serialize};

use crate::tui::app::provider_error::user_facing_error;

pub use super::types::{ChatMessage, MessageType};

/// Token usage attributed to a single chat message.
///
/// Populated from [`crate::session::SessionEvent::UsageReport`] after a
/// provider completion returns. Attached to the most recent assistant /
/// tool-call message produced by that completion.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::message::MessageUsage;
///
/// let usage = MessageUsage {
///     model: "anthropic/claude-opus-4".into(),
///     prompt_tokens: 1523,
///     completion_tokens: 284,
///     duration_ms: 4210,
/// };
/// assert_eq!(usage.prompt_tokens, 1523);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageUsage {
    /// Model that produced this completion.
    pub model: String,
    /// Input / prompt tokens charged for this completion.
    pub prompt_tokens: usize,
    /// Output / completion tokens produced by this completion.
    pub completion_tokens: usize,
    /// Wall-clock latency of the provider request in milliseconds.
    pub duration_ms: u64,
}

impl ChatMessage {
    pub fn new(message_type: MessageType, content: impl Into<String>) -> Self {
        let content = normalized_content(&message_type, content.into());
        Self {
            message_type,
            content,
            timestamp: std::time::SystemTime::now(),
            usage: None,
        }
    }
}

fn normalized_content(message_type: &MessageType, content: String) -> String {
    if matches!(message_type, MessageType::Error) {
        user_facing_error(&content)
    } else {
        content
    }
}
