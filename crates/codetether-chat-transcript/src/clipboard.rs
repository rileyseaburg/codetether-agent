//! Raw message payload formatting for clipboard export.
use codetether_chat_model::{ChatMessage, MessageType};

/// Format one message's raw payload for clipboard export.
///
/// # Arguments
/// * `msg` - Message whose payload should be copied.
/// # Returns
/// Raw text, attachment location, or tool payload with its name and kind.
/// # Examples
/// ```
/// use codetether_chat_model::{ChatMessage, MessageType};
/// use codetether_chat_transcript::clipboard_text;
/// let message = ChatMessage::new(MessageType::Assistant, "hello");
/// assert_eq!(clipboard_text(&message), "hello");
/// ```
pub fn clipboard_text(msg: &ChatMessage) -> String {
    match &msg.message_type {
        MessageType::Thinking(text) => text.clone(),
        MessageType::ToolCall {
            name, arguments, ..
        } => {
            format!("Tool call: {name}\n{arguments}")
        }
        MessageType::ToolResult { name, output, .. } => {
            format!("Tool result: {name}\n{output}")
        }
        MessageType::Image { url } => url.clone(),
        MessageType::File { path, .. } => path.clone(),
        _ => msg.content.clone(),
    }
}
