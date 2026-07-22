//! JSON decoding for one extracted tool call.

use serde_json::Value;

pub(super) fn call(raw: &str) -> Option<(String, String)> {
    let value = serde_json::from_str::<Value>(raw).ok()?;
    let name = value.get("name")?.as_str()?.trim();
    if name.is_empty() {
        return None;
    }
    let arguments = value
        .get("arguments")
        .cloned()
        .unwrap_or(Value::Object(Default::default()));
    Some((name.to_string(), serde_json::to_string(&arguments).ok()?))
}
