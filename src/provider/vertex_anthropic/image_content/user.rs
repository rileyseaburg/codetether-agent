//! Vertex user blocks retain text/thinking semantics and add native images.
use crate::provider::{ContentPart, Message};
use serde_json::{Value, json};

pub(in crate::provider::vertex_anthropic) fn parts(msg: &Message) -> Vec<Value> {
    msg.content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(json!({"type": "text", "text": text})),
            ContentPart::Thinking { text, .. } => {
                Some(json!({"type": "thinking", "thinking": text}))
            }
            ContentPart::Image { url, mime_type } => {
                Some(super::image_block(url, mime_type.as_deref()))
            }
            _ => None,
        })
        .collect()
}
