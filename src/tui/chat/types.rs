//! Chat message data types.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    User,
    Assistant,
    System,
    Error,
    ToolCall {
        name: String,
        arguments: String,
    },
    ToolResult {
        name: String,
        output: String,
        success: bool,
        #[serde(default)]
        duration_ms: Option<u64>,
    },
    Thinking(String),
    Image {
        url: String,
    },
    File {
        path: String,
        #[serde(default)]
        size: Option<u64>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub message_type: MessageType,
    pub content: String,
    pub timestamp: std::time::SystemTime,
    #[serde(default)]
    pub usage: Option<super::message::MessageUsage>,
}
