//! Check serialized Hadamard signs and widths.
use super::Index;
use anyhow::{Context, Result, ensure};
use serde_json::Value;
pub(super) fn validate(index: &Index) -> Result<()> {
    let m = &index.metadata;
    let widths = m
        .get("prism.hadamard.sign_widths")
        .and_then(Value::as_array)
        .context("Missing Hadamard widths")?;
    let mut unique = std::collections::HashSet::new();
    ensure!(
        widths
            .iter()
            .all(|v| v.as_u64().is_some_and(|n| unique.insert(n))),
        "Duplicate Hadamard widths"
    );
    let total = widths
        .iter()
        .try_fold(0u64, |sum, v| {
            let n = v.as_u64()?;
            (n > 0 && n % 1024 == 0).then_some(())?;
            sum.checked_add(n)
        })
        .context("Invalid Hadamard widths")?;
    let signs = m
        .get("prism.hadamard.sign_values")
        .and_then(Value::as_array)
        .context("Missing Hadamard signs")?;
    ensure!(
        signs.len() as u64 == total && signs.iter().all(|v| matches!(v.as_i64(), Some(-1 | 1))),
        "Invalid Hadamard signs"
    );
    Ok(())
}
