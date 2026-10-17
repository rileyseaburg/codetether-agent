//! Recursive validation of object fields and array items.

use super::validate as validate_schema;
use anyhow::{Result, bail};
use serde_json::Value;

pub(super) fn validate(value: &Value, schema: &Value) -> Result<()> {
    if let Some(object) = value.as_object() {
        required(object, schema)?;
        properties(object, schema)?;
    }
    if let (Some(items), Some(item_schema)) = (value.as_array(), schema.get("items")) {
        for (index, item) in items.iter().enumerate() {
            validate_schema(item, item_schema)
                .map_err(|error| anyhow::anyhow!("item {index}: {error}"))?;
        }
    }
    Ok(())
}

fn required(object: &serde_json::Map<String, Value>, schema: &Value) -> Result<()> {
    let required = schema
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str);
    for field in required {
        if !object.contains_key(field) {
            bail!("missing required field `{field}`");
        }
    }
    Ok(())
}

fn properties(object: &serde_json::Map<String, Value>, schema: &Value) -> Result<()> {
    let properties = schema.get("properties").and_then(Value::as_object);
    for (name, child) in object {
        if let Some(child_schema) = properties.and_then(|map| map.get(name)) {
            validate_schema(child, child_schema)
                .map_err(|error| anyhow::anyhow!("field `{name}`: {error}"))?;
        } else if schema.get("additionalProperties") == Some(&Value::Bool(false)) {
            bail!("unexpected field `{name}`");
        }
    }
    Ok(())
}
