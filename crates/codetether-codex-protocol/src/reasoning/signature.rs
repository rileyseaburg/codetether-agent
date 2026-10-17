//! Opaque persistence of OpenAI reasoning response items.

use serde_json::Value;

const PREFIX: &str = "codetether:openai-reasoning:";

pub(super) fn encode(item: &Value) -> Option<String> {
    if item
        .get("encrypted_content")
        .and_then(Value::as_str)
        .is_none()
    {
        return None;
    }
    Some(format!("{PREFIX}{item}"))
}

pub(super) fn decode(value: &str) -> Option<Value> {
    let item: Value = serde_json::from_str(value.strip_prefix(PREFIX)?).ok()?;
    if item.get("type").and_then(Value::as_str) != Some("reasoning")
        || item
            .get("encrypted_content")
            .and_then(Value::as_str)
            .is_none()
    {
        return None;
    }
    Some(item)
}

#[cfg(test)]
#[path = "signature_tests.rs"]
mod tests;
