//! Plain-text projection of provider content for exports.
use crate::buffer::push_line;
use codetether_provider_core::ContentPart;

/// Flatten provider content into text with the existing tool and attachment markers.
///
/// # Arguments
/// * `content` — Ordered parts from a provider message.
///
/// # Returns
/// Text separated by newlines in the original part order.
///
/// # Examples
/// ```
/// let parts = vec![codetether_provider_core::ContentPart::Text { text: "hello".into() }];
/// assert_eq!(codetether_chat_transcript::extract_message_text(&parts), "hello");
/// ```
pub fn extract_message_text(content: &[ContentPart]) -> String {
    let mut out = String::new();
    for part in content {
        match part {
            ContentPart::Text { text } | ContentPart::Thinking { text, .. } => {
                push_line(&mut out, text)
            }
            ContentPart::ToolCall {
                name, arguments, ..
            } => push_line(&mut out, &format!("Tool call: {name} {arguments}")),
            ContentPart::ToolResult { content, .. } => push_line(&mut out, content),
            ContentPart::Image { url, .. } => push_line(&mut out, &format!("[image: {url}]")),
            ContentPart::File { path, .. } => push_line(&mut out, &format!("[file: {path}]")),
        }
    }
    out
}
