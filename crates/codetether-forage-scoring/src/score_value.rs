//! Accept numeric scores and the original object score field.
use serde_json::Value;
pub(super) fn score_value(value: &Value) -> Option<f64> {
    match value {
        Value::Number(number) => number.as_f64(),
        Value::Object(map) => map.get("ok").and_then(Value::as_f64),
        _ => None,
    }
}
