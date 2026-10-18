//! Classification of chat messages rendered in the tool panel.

use codetether_chat_model::MessageType;

/// Reports whether a chat message belongs in the compact tool-activity panel.
///
/// # Arguments
///
/// * `message_type` - Chat message classification to inspect.
///
/// # Returns
///
/// `true` for tool calls and tool results.
///
/// # Examples
///
/// ```rust
/// use codetether_chat_model::MessageType;
/// use codetether_tool_panel::is_tool_activity;
///
/// assert!(!is_tool_activity(&MessageType::Thinking("working".to_string())));
/// ```
pub fn is_tool_activity(message_type: &MessageType) -> bool {
    matches!(
        message_type,
        MessageType::ToolCall { .. } | MessageType::ToolResult { .. }
    )
}
