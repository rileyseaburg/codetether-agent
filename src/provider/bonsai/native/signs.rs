//! Load explicitly serialized rotation signs onto the inference device.
use super::Index;
use anyhow::{Context, Result};
use candle_core::{Device, Tensor};
use std::collections::HashMap;
pub(super) fn load(index: &Index, device: &Device) -> Result<HashMap<usize, Tensor>> {
    let widths = index.metadata["prism.hadamard.sign_widths"]
        .as_array()
        .context("Missing widths")?;
    let values = index.metadata["prism.hadamard.sign_values"]
        .as_array()
        .context("Missing signs")?;
    let mut result = HashMap::new();
    let mut offset = 0;
    for width in widths {
        let width = usize::try_from(width.as_u64().context("Invalid sign width")?)?;
        let signs = values[offset..offset + width]
            .iter()
            .map(|v| v.as_i64().unwrap() as f32)
            .collect::<Vec<_>>();
        result.insert(width, Tensor::from_vec(signs, width, device)?);
        offset += width;
    }
    Ok(result)
}
