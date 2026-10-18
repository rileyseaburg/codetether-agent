//! Text-only projection used to size and summarize a message.
use codetether_provider_core::{ContentPart, Message};

/// Concatenate the textual content of `msg`, skipping non-text parts.
pub(super) fn extract_message_text(msg: &Message) -> String {
    let mut buf = String::new();
    for part in &msg.content {
        if let ContentPart::Text { text } = part {
            if !buf.is_empty() {
                buf.push('\n');
            }
            buf.push_str(text);
        }
    }
    buf
}
