//! Remove volatile fields recursively without altering persistent input values.
use serde_json::Value;

pub(super) fn filter_volatile(value: &Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut out = serde_json::Map::new();
            for (k, v) in map {
                if matches!(k.as_str(), "tool_call_id" | "instruction" | "update") {
                    continue;
                }
                out.insert(k.clone(), filter_volatile(v));
            }
            Value::Object(out)
        }
        Value::Array(arr) => Value::Array(arr.iter().map(filter_volatile).collect()),
        _ => value.clone(),
    }
}
