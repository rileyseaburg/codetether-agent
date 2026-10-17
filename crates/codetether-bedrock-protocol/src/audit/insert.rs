//! Insert synthesized tool results without losing existing user content.
use serde_json::{Value, json};

pub(super) fn insert_results(messages: &mut Vec<Value>, index: usize, ids: &[String]) {
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
    for (offset, id) in ids.iter().enumerate() {
        content.insert(offset, interrupted(id));
    }
}

fn interrupted(id: &str) -> Value {
    json!({"toolResult": {
        "toolUseId": id,
        "status": "error",
        "content": [{"text": "(tool call interrupted; no result recorded)"}],
    }})
}
