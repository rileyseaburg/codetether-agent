//! Render provider message parts into searchable turn text.
use codetether_provider_core::{ContentPart, Message};

/// Render all message parts in their original order.
///
/// # Arguments
/// * `msg` - A canonical transcript message.
/// # Returns
/// Searchable text with explicit markers for tool, image, file and thinking parts.
/// # Examples
/// ```rust
/// use codetether_provider_core::{Message,Role,ContentPart};
/// let message=Message{role:Role::User,content:vec![ContentPart::Text{text:"hello".into()}]};
/// assert_eq!(codetether_history_files::render_turn(&message),"hello");
/// ```
pub fn render_turn(msg: &Message) -> String {
    let mut buf = String::new();
    for part in &msg.content {
        if !buf.is_empty() {
            buf.push_str("\n\n");
        }
        match part {
            ContentPart::Text { text } => buf.push_str(text),
            ContentPart::ToolResult {
                tool_call_id,
                content,
            } => {
                buf.push_str(&format!("[tool_result tool_call_id={tool_call_id}]\n"));
                buf.push_str(content);
            }
            ContentPart::ToolCall {
                name, arguments, ..
            } => buf.push_str(&format!("[tool_call {name}]\n{arguments}")),
            ContentPart::Image { url, .. } => buf.push_str(&format!("[image {url}]")),
            ContentPart::File { path, .. } => buf.push_str(&format!("[file {path}]")),
            ContentPart::Thinking { text, .. } => buf.push_str(&format!("[thinking]\n{text}")),
        }
    }
    buf
}
