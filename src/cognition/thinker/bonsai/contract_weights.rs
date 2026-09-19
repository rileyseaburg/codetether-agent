//! Bind forward and inverse rotations to real tensor names and dimensions.
use super::Index;
use anyhow::{Context, Result, ensure};
use serde_json::Value;
use std::collections::HashSet;
pub(super) fn validate(index: &Index) -> Result<()> {
    let mut rotated = HashSet::new();
    let widths = index
        .metadata
        .get("prism.hadamard.sign_widths")
        .and_then(Value::as_array)
        .context("Missing sign widths")?;
    for key in [
        "prism.hadamard.weight_names",
        "prism.hadamard.inverse_weight_names",
    ] {
        let names = index
            .metadata
            .get(key)
            .and_then(Value::as_array)
            .context("Missing Hadamard tensor mapping")?;
        for name in names {
            let name = name.as_str().context("Invalid rotated weight name")?;
            ensure!(rotated.insert(name), "Duplicate rotated tensor {name}");
            let tensor = index
                .tensors
                .get(name)
                .context("Rotation references a missing tensor")?;
            ensure!(
                tensor.kind == 142 && tensor.shape.len() == 2,
                "Rotation requires a PQ2_0 matrix"
            );
            ensure!(
                widths
                    .iter()
                    .any(|v| v.as_u64() == Some(tensor.shape[0] as u64)),
                "Missing signs for {name}"
            );
        }
    }
    for (name, tensor) in &index.tensors {
        ensure!(
            tensor.kind != 142 || rotated.contains(name.as_str()),
            "Unaccounted PQ2_0 transform for {name}"
        );
    }
    Ok(())
}
