//! Model-specific Bedrock conversation-shape compatibility.

use serde_json::{Value, json};

/// Append a user continuation when a model rejects assistant prefills.
pub(super) fn ensure_user_final_turn(messages: &mut Vec<Value>, model_id: &str) {
    if !rejects_assistant_prefill(model_id) {
        return;
    }
    let ends_with_assistant = messages
        .last()
        .and_then(|message| message.get("role"))
        .and_then(Value::as_str)
        == Some("assistant");
    if ends_with_assistant {
        messages.push(json!({
            "role": "user",
            "content": [{"text": "Continue."}]
        }));
    }
}

fn rejects_assistant_prefill(model_id: &str) -> bool {
    model_id.to_ascii_lowercase().contains("claude-opus-5")
}
