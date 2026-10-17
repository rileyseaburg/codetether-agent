//! Typed scalar accessors for validated GGUF metadata.
use super::Index;
use anyhow::{Context, Result};
pub(super) fn integer(index: &Index, key: &str) -> Result<usize> {
    Ok(usize::try_from(
        index
            .metadata
            .get(key)
            .and_then(serde_json::Value::as_u64)
            .with_context(|| format!("Missing {key}"))?,
    )?)
}
pub(super) fn float(index: &Index, key: &str) -> Result<f64> {
    index
        .metadata
        .get(key)
        .and_then(serde_json::Value::as_f64)
        .with_context(|| format!("Missing {key}"))
}
