//! Tool-use identifier extraction from Bedrock content blocks.

use serde_json::Value;

pub(super) fn tool_use_ids(message: &Value) -> Vec<String> {
    message
        .get("content")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|part| part.pointer("/toolUse/toolUseId")?.as_str())
        .map(String::from)
        .collect()
}
