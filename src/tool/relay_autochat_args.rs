//! Optional string extraction for relay actions.

use serde_json::Value;

pub(super) fn string(params: &Value, name: &str) -> Option<String> {
    params.get(name).and_then(Value::as_str).map(String::from)
}
