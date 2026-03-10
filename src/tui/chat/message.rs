use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    User,
    Assistant,
    System,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub message_type: MessageType,
    pub content: String,
    pub timestamp: std::time::SystemTime,
}

impl ChatMessage {
    pub fn new(message_type: MessageType, content: impl Into<String>) -> Self {
        Self {
            message_type,
            content: content.into(),
            timestamp: std::time::SystemTime::now(),
        }
    }
}
