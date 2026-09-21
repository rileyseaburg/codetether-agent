//! Prevent generation of GGUF padding/unused vocabulary IDs absent from the tokenizer.
use super::Index;
use anyhow::{Context, Result};
use candle_core::{Device, Tensor};
pub(super) fn load(index: &Index, device: &Device) -> Result<Tensor> {
    let types = index
        .metadata
        .get("tokenizer.ggml.token_type")
        .and_then(serde_json::Value::as_array)
        .context("Missing token types")?;
    anyhow::ensure!(
        types.iter().all(|v| matches!(v.as_u64(), Some(1..=6))),
        "Invalid GGUF token type"
    );
    let mask = types
        .iter()
        .map(|value| {
            if value.as_u64() == Some(5) {
                f32::NEG_INFINITY
            } else {
                0.0
            }
        })
        .collect::<Vec<_>>();
    Ok(Tensor::from_vec(mask, types.len(), device)?)
}
