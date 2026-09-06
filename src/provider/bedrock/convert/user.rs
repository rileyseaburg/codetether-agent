//! Bedrock user text and image conversion.
use crate::provider::{ContentPart, Message};
use serde_json::{Value, json};

pub(super) fn append_user(msg: &Message, api_messages: &mut Vec<Value>) {
    let parts = msg
        .content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } if !text.trim().is_empty() => Some(json!({"text": text})),
            ContentPart::Image { url, mime_type } => {
                Some(super::image::block(url, mime_type.as_deref()))
            }
            _ => None,
        })
        .collect();
    super::merge::append(api_messages, "user", parts);
}
