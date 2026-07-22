//! JSON value type checks used by tool-call validation.

use anyhow::{Result, bail};
use serde_json::Value;

pub(super) fn validate(value: &Value, schema: &Value) -> Result<()> {
    let Some(expected) = schema.get("type") else {
        return Ok(());
    };
    let matches = if let Some(name) = expected.as_str() {
        matches_type(value, name)
    } else if let Some(names) = expected.as_array() {
        names
            .iter()
            .filter_map(Value::as_str)
            .any(|name| matches_type(value, name))
    } else {
        true
    };
    if !matches {
        let label = expected.as_str().unwrap_or("one of the advertised types");
        bail!("expected {label}, received {}", kind(value));
    }
    Ok(())
}

fn matches_type(value: &Value, expected: &str) -> bool {
    match expected {
        "object" => value.is_object(),
        "array" => value.is_array(),
        "string" => value.is_string(),
        "integer" => value.as_i64().is_some() || value.as_u64().is_some(),
        "number" => value.is_number(),
        "boolean" => value.is_boolean(),
        "null" => value.is_null(),
        _ => true,
    }
}

fn kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
