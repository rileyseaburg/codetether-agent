//! Tool-result ordering, synthesis, and orphan removal.

use serde_json::{Value, json};

pub(super) fn put_required_results_first(message: &mut Value, ids: &[String]) {
    let Some(content) = message["content"].as_array_mut() else {
        return;
    };
    let mut remaining = std::mem::take(content);
    let mut ordered = Vec::with_capacity(remaining.len().max(ids.len()));
    for id in ids {
        let matching = remaining
            .iter()
            .position(|part| tool_result_id(part) == Some(id.as_str()))
            .map(|index| remaining.remove(index))
            .unwrap_or_else(|| interrupted_result(id));
        ordered.push(matching);
    }
    remaining.retain(|part| tool_result_id(part).is_none());
    ordered.append(&mut remaining);
    *content = ordered;
}

pub(super) fn drop_orphan_results(message: &mut Value) {
    let Some(content) = message.get_mut("content").and_then(Value::as_array_mut) else {
        return;
    };
    let original_len = content.len();
    content.retain(|part| tool_result_id(part).is_none());
    if content.is_empty() && original_len > 0 {
        content.push(json!({"text": "(stale tool result omitted)"}));
    }
}

fn tool_result_id(part: &Value) -> Option<&str> {
    part.pointer("/toolResult/toolUseId")?.as_str()
}

fn interrupted_result(id: &str) -> Value {
    json!({"toolResult": {"toolUseId": id, "status": "error", "content": [
        {"text": "(tool call interrupted; no result recorded)"}
    ]}})
}
