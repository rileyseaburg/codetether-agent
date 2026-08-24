//! Removal of local authorization metadata before remote MCP transport.

use serde_json::Value;

pub(super) fn sanitize(input: Value) -> Value {
    match input {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .filter(|(key, _)| key != "approval_id" && !key.starts_with("__ct_"))
                .map(|(key, value)| (key, sanitize(value)))
                .collect(),
        ),
        Value::Array(values) => Value::Array(values.into_iter().map(sanitize).collect()),
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => input,
    }
}

#[cfg(test)]
#[test]
fn strips_nested_approval_and_runtime_metadata() {
    let input = serde_json::json!({"nested": {"approval_id": "secret", "__ct_session_id": "s", "value": 1}});
    assert_eq!(sanitize(input), serde_json::json!({"nested": {"value": 1}}));
}
