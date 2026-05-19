use crate::provider::{ContentPart, Message};
use serde_json::Value;

/// Apply repaired reasoning_content back to the internal Message.
pub fn apply_ds_repair(msg: &mut Message, repaired: &Value) {
    let new_reasoning = repaired
        .get("reasoning_content")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    msg.content
        .retain(|p| !matches!(p, ContentPart::Thinking { .. }));
    if !new_reasoning.is_empty() {
        msg.content.push(ContentPart::Thinking {
            text: new_reasoning.to_string(),
        });
    }
}
