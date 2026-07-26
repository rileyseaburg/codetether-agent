//! Conversation image context supplied to image-generation calls.

use crate::provider::ContentPart;
use crate::session::Session;
use serde_json::{Value, json};

const IMAGE_TOOLS: [&str; 2] = ["image_gen", "imagegen"];
const MAX_IMAGES: usize = 5;

pub(super) fn enrich(name: &str, call_id: &str, session: &Session, mut input: Value) -> Value {
    if !IMAGE_TOOLS.contains(&name) {
        return input;
    }
    let Value::Object(fields) = &mut input else {
        return input;
    };
    fields.insert("__ct_tool_call_id".into(), json!(call_id));
    fields.insert("__ct_recent_images".into(), json!(recent(session)));
    input
}

fn recent(session: &Session) -> Vec<String> {
    let mut images = session
        .messages
        .iter()
        .rev()
        .flat_map(|message| message.content.iter().rev())
        .filter_map(|part| match part {
            ContentPart::Image { url, .. } => Some(url.clone()),
            _ => None,
        })
        .take(MAX_IMAGES)
        .collect::<Vec<_>>();
    images.reverse();
    images
}
