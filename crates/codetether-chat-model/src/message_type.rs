//! Serializable role and payload kind for one displayed chat message.
use serde::{Deserialize, Serialize};

/// Chat role or structured payload kind, retaining the existing serialized shape.
/// # Examples
/// ```
/// use codetether_chat_model::MessageType;
/// let kind = MessageType::User;
/// assert!(matches!(kind, MessageType::User));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// User input.
    User,
    /// Assistant response.
    Assistant,
    /// System information.
    System,
    /// Error text.
    Error,
    /// Requested tool and arguments.
    ToolCall {
        /// Tool name.
        name: String,
        /// Serialized tool arguments.
        arguments: String,
    },
    /// Tool output and optional duration.
    ToolResult {
        /// Tool name.
        name: String,
        /// Raw tool output.
        output: String,
        /// Tool success flag.
        success: bool,
        #[serde(default)]
        /// Optional wall-clock duration in milliseconds.
        duration_ms: Option<u64>,
    },
    /// Reasoning or progress text.
    Thinking(String),
    /// Image URL.
    Image {
        /// Image URL or data URL.
        url: String,
    },
    /// File path and optional size.
    File {
        /// File path.
        path: String,
        #[serde(default)]
        /// Optional size in bytes.
        size: Option<u64>,
    },
}
