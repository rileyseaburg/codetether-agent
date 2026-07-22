//! Validation of JSON Schema branch combinators.

use super::validate as validate_schema;
use anyhow::{Result, bail};
use serde_json::Value;

pub(super) fn validate(value: &Value, schema: &Value) -> Result<()> {
    if let Some(branches) = schema.get("allOf").and_then(Value::as_array) {
        for branch in branches {
            validate_schema(value, branch)?;
        }
    }
    if let Some(branches) = schema.get("anyOf").and_then(Value::as_array)
        && !branches
            .iter()
            .any(|branch| validate_schema(value, branch).is_ok())
    {
        bail!("value does not match any allowed schema branch");
    }
    if let Some(branches) = schema.get("oneOf").and_then(Value::as_array) {
        let matches = branches
            .iter()
            .filter(|branch| validate_schema(value, branch).is_ok())
            .count();
        if matches != 1 {
            bail!("value must match exactly one schema branch; matched {matches}");
        }
    }
    Ok(())
}
