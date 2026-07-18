//! Classification of chat messages rendered in the tool panel.

use crate::tui::chat::message::MessageType;

/// Reports whether a chat message belongs in the compact tool-activity panel.
///
/// # Arguments
///
/// * `message_type` - Chat message classification to inspect.
///
/// # Returns
///
/// `true` for tool calls, tool results, and thinking activity.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::message::MessageType;
/// use codetether_agent::tui::ui::tool_panel::is_tool_activity;
///
/// assert!(is_tool_activity(&MessageType::Thinking("working".to_string())));
/// ```
pub fn is_tool_activity(message_type: &MessageType) -> bool {
    matches!(
        message_type,
        MessageType::ToolCall { .. } | MessageType::ToolResult { .. } | MessageType::Thinking(_)
    )
}
