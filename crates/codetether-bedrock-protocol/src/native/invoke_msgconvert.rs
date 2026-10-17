//! Remap Bedrock Converse message blocks into native Anthropic Messages
//! content blocks for the InvokeModel path.
//!
//! Converse emits `{"text":...}`, `{"toolUse":{...}}`, and
//! `{"toolResult":{...}}`; native Anthropic requires `type`-tagged blocks:
//! `{"type":"text",...}`, `{"type":"tool_use",...}`,
//! `{"type":"tool_result",...}`.

use super::invoke_blocks::remap_block;
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

/// Remap a full Converse `messages` array into native Anthropic messages.
///
/// # Arguments
/// * `messages` - Already-normalized Converse messages.
///
/// # Returns
/// Messages using native Anthropic content tags.
///
/// # Examples
/// ```
/// assert!(codetether_bedrock_protocol::native::invoke_msgconvert::remap_messages_native(&[]).is_empty());
/// ```
pub fn remap_messages_native(messages: &[Value]) -> Vec<Value> {
    messages.iter().map(remap_message).collect()
}
