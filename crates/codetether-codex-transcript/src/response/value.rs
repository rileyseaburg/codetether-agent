//! Render tool arguments and outputs with existing null/string semantics.
use serde_json::Value;

pub(super) fn value_to_string(value: Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::String(text) => Some(text),
        other => Some(other.to_string()),
    }
}
