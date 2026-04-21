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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub message_type: MessageType,
    pub content: String,
    pub timestamp: std::time::SystemTime,
    /// Per-message token usage, populated after the provider returns.
    /// `None` for messages that do not correspond to a provider call
    /// (user input, system notices, tool results).
    #[serde(default)]
    pub usage: Option<MessageUsage>,
}

impl ChatMessage {
    pub fn new(message_type: MessageType, content: impl Into<String>) -> Self {
        Self {
            message_type,
            content: content.into(),
            timestamp: std::time::SystemTime::now(),
            usage: None,
        }
    }
}
