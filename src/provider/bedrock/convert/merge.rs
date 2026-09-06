//! Preserve Bedrock role alternation by merging adjacent same-role messages.
use serde_json::{Value, json};

pub(super) fn append(messages: &mut Vec<Value>, role: &str, content: Vec<Value>) {
    if content.is_empty() {
        return;
    }
    if let Some(last) = messages.last_mut()
        && last.get("role").and_then(Value::as_str) == Some(role)
        && let Some(parts) = last.get_mut("content").and_then(Value::as_array_mut)
    {
        parts.extend(content);
        return;
    }
    messages.push(json!({"role": role, "content": content}));
}
