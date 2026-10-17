//! Conversation image context supplied to image-generation calls.

use codetether_provider_core::{ContentPart, Message};
use serde_json::{Value, json};

const IMAGE_TOOLS: [&str; 2] = ["image_gen", "imagegen"];
const MAX_IMAGES: usize = 5;

/// Attach recent conversation image URLs for image-generation tools.
///
/// # Arguments
/// * `name` — Tool identifier.
/// * `call_id` — Identifier of the current tool call.
/// * `messages` — Conversation history in chronological order.
/// * `input` — Parsed tool arguments.
/// # Returns
/// Arguments with image context for supported image tools, otherwise unchanged.
/// # Examples
/// ```
/// let input = serde_json::json!({"prompt": "draw a tree"});
/// let result = codetether_agent_core::execution::images::enrich("image_gen", "call", &[], input);
/// assert_eq!(result["__ct_tool_call_id"], "call");
/// ```
pub fn enrich(name: &str, call_id: &str, messages: &[Message], mut input: Value) -> Value {
    if !IMAGE_TOOLS.contains(&name) {
        return input;
    }
    let Value::Object(fields) = &mut input else {
        return input;
    };
    fields.insert("__ct_tool_call_id".into(), json!(call_id));
    fields.insert("__ct_recent_images".into(), json!(recent(messages)));
    input
}

fn recent(messages: &[Message]) -> Vec<String> {
    let mut images = messages
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
