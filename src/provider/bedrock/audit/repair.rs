//! In-place repair of unpaired toolUse blocks in a final request body.

use serde_json::{Value, json};

/// Ensure every `toolUse` in `body["messages"]` has a following `toolResult`.
///
/// Returns the ids that had to be synthesized, so callers can log the fact
/// that a malformed body was caught locally rather than by Bedrock.
pub(in crate::provider::bedrock) fn enforce(body: &mut Value) -> Vec<String> {
    let Some(messages) = body.get_mut("messages").and_then(Value::as_array_mut) else {
        return Vec::new();
    };
    let mut synthesized = Vec::new();
    while let Some((index, missing)) = super::scan::unpaired(messages) {
        insert_results(messages, index + 1, &missing);
        synthesized.extend(missing);
    }
    if !synthesized.is_empty() {
        tracing::warn!(
            provider = "bedrock",
            ids = ?synthesized,
            "repaired unpaired toolUse blocks before send"
        );
    }
    synthesized
}

fn insert_results(messages: &mut Vec<Value>, index: usize, ids: &[String]) {
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
