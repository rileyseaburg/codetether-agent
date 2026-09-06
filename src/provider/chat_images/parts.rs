use super::ContentPart;
use serde_json::{Value, json};

pub(super) fn user(content: &[ContentPart]) -> Vec<Value> {
    content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(json!({"type": "text", "text": text})),
            ContentPart::Image { url, .. } => Some(image(url)),
            _ => None,
        })
        .collect()
}

pub(super) fn image(url: &str) -> Value {
    json!({"type": "image_url", "image_url": {"url": url}})
}
