//! Displayed chat message data and construction.
use crate::{MessageType, MessageUsage};
use serde::{Deserialize, Serialize};

/// A timestamped chat payload with optional completion usage.
/// # Examples
/// ```
/// use codetether_chat_model::{ChatMessage, MessageType};
/// let message = ChatMessage::new(MessageType::User, "hello");
/// assert_eq!(message.content, "hello");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Message role and structured payload.
    pub message_type: MessageType,
    /// Display text associated with the payload.
    pub content: String,
    /// Creation time.
    pub timestamp: std::time::SystemTime,
    /// Per-message token usage, populated after the provider returns.
    /// `None` for messages that do not correspond to a provider call
    /// (user input, system notices, tool results).
    #[serde(default)]
    pub usage: Option<MessageUsage>,
}

impl ChatMessage {
    /// Construct a message with the current time and no recorded usage.
    /// # Arguments
    /// * `message_type` - Role and structured payload.
    /// * `content` - Display text.
    /// # Returns
    /// A new timestamped message.
    /// # Examples
    /// ```
    /// use codetether_chat_model::{ChatMessage, MessageType};
    /// assert!(ChatMessage::new(MessageType::System, "ready").usage.is_none());
    /// ```
    pub fn new(message_type: MessageType, content: impl Into<String>) -> Self {
        Self {
            message_type,
            content: content.into(),
            timestamp: std::time::SystemTime::now(),
            usage: None,
        }
    }
}
