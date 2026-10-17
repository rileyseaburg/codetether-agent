//! Required conversation-anchor normalization for OpenAI Responses requests.

use serde_json::{Value, json};

const CONTINUATION: &str = "Continue from the available conversation context.";

/// Preserve converted input or add a neutral continuation message when it is empty.
pub(crate) fn ensure(mut input: Vec<Value>) -> Vec<Value> {
    if input.is_empty() {
        input.push(json!({
            "type": "message",
            "role": "user",
            "content": [{"type": "input_text", "text": CONTINUATION}]
        }));
    }
    input
}

#[cfg(test)]
#[path = "request_anchor/tests.rs"]
mod tests;
