//! Synthetic native `tool_result` insertion for unanswered calls.

use serde_json::{Value, json};

pub(super) fn results(messages: &mut Vec<Value>, index: usize, missing: &[String]) {
    let is_user = messages
        .get(index)
        .and_then(|message| message.get("role"))
        .and_then(Value::as_str)
        == Some("user");
    if !is_user {
        messages.insert(index, json!({"role": "user", "content": []}));
    } else if !messages[index]["content"].is_array() {
        messages[index]["content"] = json!([]);
    }
    let Some(content) = messages[index]["content"].as_array_mut() else {
        return;
    };
    for (offset, id) in missing.iter().enumerate() {
        content.insert(offset, interrupted(id));
    }
}

fn interrupted(id: &str) -> Value {
    json!({
        "type": "tool_result",
        "tool_use_id": id,
        "is_error": true,
        "content": [{"type": "text", "text": "(tool call interrupted; no result recorded)"}],
    })
}
