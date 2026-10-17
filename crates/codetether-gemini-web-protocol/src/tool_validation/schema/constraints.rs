//! Numeric, string, and collection bounds from advertised schemas.

use anyhow::{Result, bail};
use serde_json::Value;

pub(super) fn validate(value: &Value, schema: &Value) -> Result<()> {
    if let Some(number) = value.as_f64() {
        if let Some(minimum) = schema.get("minimum").and_then(Value::as_f64)
            && number < minimum
        {
            bail!("value is below minimum {minimum}");
        }
        if let Some(maximum) = schema.get("maximum").and_then(Value::as_f64)
            && number > maximum
        {
            bail!("value exceeds maximum {maximum}");
        }
    }
    if let Some(text) = value.as_str() {
        length(text.chars().count(), schema, "Length")?;
    }
    if let Some(items) = value.as_array() {
        length(items.len(), schema, "Items")?;
    }
    Ok(())
}

fn length(actual: usize, schema: &Value, suffix: &str) -> Result<()> {
    let minimum = schema.get(format!("min{suffix}")).and_then(Value::as_u64);
    let maximum = schema.get(format!("max{suffix}")).and_then(Value::as_u64);
    if minimum.is_some_and(|bound| actual < bound as usize) {
        bail!(
            "length {actual} is below minimum {}",
            minimum.unwrap_or_default()
        );
    }
    if maximum.is_some_and(|bound| actual > bound as usize) {
        bail!(
            "length {actual} exceeds maximum {}",
            maximum.unwrap_or_default()
        );
    }
    Ok(())
}
