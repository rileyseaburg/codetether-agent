//! Remap Bedrock Converse message blocks into native Anthropic Messages
//! content blocks for the InvokeModel path.
//!
//! Converse emits `{"text":...}`, `{"toolUse":{...}}`, and
//! `{"toolResult":{...}}`; native Anthropic requires `type`-tagged blocks:
//! `{"type":"text",...}`, `{"type":"tool_use",...}`,
//! `{"type":"tool_result",...}`.

use serde_json::{Value, json};

/// Remap one Converse message into a native Anthropic message.
fn remap_message(msg: &Value) -> Value {
    let role = msg.get("role").cloned().unwrap_or(json!("user"));
    let content = msg
        .get("content")
        .and_then(Value::as_array)
        .map(|arr| arr.iter().map(remap_block).collect::<Vec<_>>())
        .unwrap_or_default();
    json!({"role": role, "content": content})
}

/// Remap a single Converse content block into a native block.
fn remap_block(block: &Value) -> Value {
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

/// Remap a full Converse `messages` array into native Anthropic messages.
pub fn remap_messages_native(messages: &[Value]) -> Vec<Value> {
    messages.iter().map(remap_message).collect()
}
