//! Unpaired native `tool_use` detection.

use serde_json::Value;

pub(super) fn unpaired(messages: &[Value]) -> Option<(usize, Vec<String>)> {
    for (index, message) in messages.iter().enumerate() {
        let declared = ids(message, "tool_use", "id");
        if declared.is_empty() {
            continue;
        }
        let answered = messages
            .get(index + 1)
            .map(|next| ids(next, "tool_result", "tool_use_id"))
            .unwrap_or_default();
        let missing: Vec<String> = declared
            .into_iter()
            .filter(|id| !answered.contains(id))
            .collect();
        if !missing.is_empty() {
            return Some((index, missing));
        }
    }
    None
}

fn ids(message: &Value, block_type: &str, key: &str) -> Vec<String> {
    message
        .get("content")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|part| part.get("type").and_then(Value::as_str) == Some(block_type))
        .filter_map(|part| part.get(key)?.as_str())
        .map(String::from)
        .collect()
}
