//! Map one Converse block to the native Anthropic representation.
use serde_json::{Value, json};

/// Remap a single Converse content block into a native block.
pub(super) fn remap_block(block: &Value) -> Value {
    if let Some(text) = block
        .get("text")
        .filter(|_| block.get("toolResult").is_none())
    {
        return json!({"type": "text", "text": text});
    }
    if let Some(tu) = block.get("toolUse") {
        return json!({
            "type": "tool_use",
            "id": tu.get("toolUseId").cloned().unwrap_or(Value::Null),
            "name": tu.get("name").cloned().unwrap_or(Value::Null),
            "input": tu.get("input").cloned().unwrap_or(json!({})),
        });
    }
    if let Some(tr) = block.get("toolResult") {
        let text = tr
            .get("content")
            .and_then(Value::as_array)
            .and_then(|c| c.first())
            .and_then(|f| f.get("text"))
            .cloned()
            .unwrap_or(json!(""));
        return json!({
            "type": "tool_result",
            "tool_use_id": tr.get("toolUseId").cloned().unwrap_or(Value::Null),
            "content": [{"type": "text", "text": text}],
        });
    }
    block.clone()
}
