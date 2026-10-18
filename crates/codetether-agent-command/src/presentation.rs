//! Convert child-tool outcomes into chat messages and status text.
use codetether_chat_model::{ChatMessage, MessageType};
use codetether_tool_core::ToolResult;
/// Prepare a user-visible tool outcome with its command status.
/// # Arguments
/// Receives the tool result and success/failure labels.
/// # Returns
/// The transcript message and status text to apply together.
/// # Examples
/// ```
/// let (_, status) = codetether_agent_command::presentation::prepare(
///     &codetether_tool_core::ToolResult::success("sent"), "Delivered".into(), "Failed");
/// assert_eq!(status, "Delivered");
/// ```
pub fn prepare(result: &ToolResult, success: String, failure: &str) -> (ChatMessage, String) {
    let kind = if result.success {
        MessageType::System
    } else {
        MessageType::Error
    };
    let message = ChatMessage::new(kind, result.output.clone());
    let status = if result.success {
        success
    } else {
        format!("{failure}: {}", result.output)
    };
    (message, status)
}
