//! Normalization of JSON Schema `required` assertions.

use serde_json::{Map, Value};
use std::collections::HashSet;

pub(super) fn normalize(object: &mut Map<String, Value>) {
    if object.get("type").and_then(Value::as_str) != Some("object") {
        object.remove("required");
        return;
    }
    let Some(properties) = object.get("properties").and_then(Value::as_object) else {
        return;
    };
    let names = properties.keys().cloned().collect::<HashSet<_>>();
    if let Some(required) = object.get_mut("required").and_then(Value::as_array_mut) {
        required.retain(|name| name.as_str().is_some_and(|name| names.contains(name)));
    }
}
