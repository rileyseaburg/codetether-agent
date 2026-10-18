//! Safe string extraction for structured A2A activity fields.

use serde_json::Value;

pub(super) fn text<'a>(event: &'a Value, key: &str) -> Option<&'a str> {
    event.get(key).and_then(Value::as_str)
}

pub(super) fn display(value: &Value) -> String {
    value
        .as_str()
        .map_or_else(|| value.to_string(), ToString::to_string)
}
