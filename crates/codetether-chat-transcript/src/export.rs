//! Shareable conversation formatting without internal message rows.
use codetether_chat_model::{ChatMessage, MessageType};

/// Format user and assistant turns as a shareable plain-text transcript.
///
/// # Arguments
/// * `messages` - Display messages in chronological order.
/// # Returns
/// Trimmed nonempty conversation turns with role headings; internal rows are omitted.
/// # Examples
/// ```
/// use codetether_chat_transcript::build_transcript;
/// assert_eq!(build_transcript(&[]), "");
/// ```
pub fn build_transcript(messages: &[ChatMessage]) -> String {
    let mut out = String::new();
    for msg in messages {
        let role = match &msg.message_type {
            MessageType::User => "You",
            MessageType::Assistant => "Assistant",
            _ => continue,
        };
        let content = msg.content.trim();
        if content.is_empty() {
            continue;
        }
        if !out.is_empty() {
            out.push_str("\n\n");
        }
        out.push_str("=== ");
        out.push_str(role);
        out.push_str(" ===\n");
        out.push_str(content);
    }
    out
}
