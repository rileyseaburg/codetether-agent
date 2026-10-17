//! Chat message wire representation and construction.

use serde::{Deserialize, Serialize};

/// Serializable user, assistant, tool-call, and tool-result chat events.
///
/// # Examples
/// ```
/// use codetether_event_stream::ChatEvent;
/// let event = ChatEvent::UserMessage { content: "hello".into(), timestamp: 0 };
/// assert!(matches!(event, ChatEvent::UserMessage { timestamp: 0, .. }));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatEvent {
    /// User message
    UserMessage { content: String, timestamp: i64 },
    /// Assistant message
    AssistantMessage { content: String, timestamp: i64 },
    /// Tool call
    ToolCall {
        tool_name: String,
        arguments: serde_json::Value,
        timestamp: i64,
    },
    /// Tool result
    ToolResult {
        tool_name: String,
        result: String,
        success: bool,
        timestamp: i64,
    },
}

impl ChatEvent {
    /// Create a tool result event timestamped with the current UTC time.
    ///
    /// # Arguments
    /// `tool_name`, `success`, and `result` describe the result. Workspace,
    /// session, duration, and message-count arguments are retained for compatibility.
    ///
    /// # Returns
    /// A tool-result event containing the supplied outcome and current timestamp.
    ///
    /// # Examples
    /// ```
    /// use codetether_event_stream::ChatEvent;
    /// let event = ChatEvent::tool_result(".".into(), "run".into(), "read", true, 0, "ok", 1);
    /// assert!(matches!(event, ChatEvent::ToolResult { success: true, .. }));
    /// ```
    pub fn tool_result(
        workspace: std::path::PathBuf,
        _session_id: String,
        tool_name: &str,
        success: bool,
        _duration_ms: u64,
        result: &str,
        _message_count: u64,
    ) -> Self {
        let _ = workspace; // Suppress unused warning
        Self::ToolResult {
            tool_name: tool_name.to_string(),
            result: result.to_string(),
            success,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    /// Serialize the event as JSON, returning an empty string if serialization fails.
    ///
    /// # Examples
    /// ```
    /// use codetether_event_stream::ChatEvent;
    /// let event = ChatEvent::AssistantMessage { content: "hi".into(), timestamp: 0 };
    /// assert!(event.to_json().contains("assistant_message"));
    /// ```
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}
