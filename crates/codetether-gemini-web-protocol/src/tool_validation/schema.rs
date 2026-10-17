//! Deterministic validation for the tool schemas CodeTether advertises.

mod combinators;
mod constraints;
mod fields;
mod types;

use anyhow::{Result, bail};
use serde_json::Value;

pub(super) fn validate(value: &Value, schema: &Value) -> Result<()> {
    combinators::validate(value, schema)?;
    types::validate(value, schema)?;
    constraints::validate(value, schema)?;
    if let Some(allowed) = schema.get("enum").and_then(Value::as_array)
        && !allowed.contains(value)
    {
        bail!("value is not in the advertised enum");
    }
    fields::validate(value, schema)
}

#[cfg(test)]
#[path = "schema_tests.rs"]
mod tests;
