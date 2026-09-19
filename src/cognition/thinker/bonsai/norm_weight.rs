//! Small unquantized norm/recurrence tensors from the validated GGUF directory.
use super::Index;
use anyhow::{Context, Result};
use candle_core::{DType, Device, Tensor};
use std::io::{Read, Seek};
pub(super) fn load<R: Read + Seek>(
    index: &Index,
    reader: &mut R,
    name: &str,
    device: &Device,
) -> Result<Tensor> {
    let info = index
        .tensors
        .get(name)
        .with_context(|| format!("Missing {name}"))?;
    anyhow::ensure!(
        info.kind == 0 || info.kind == 30,
        "Expected an unquantized scalar tensor"
    );
    let mut dims = info.shape.clone();
    dims.reverse();
    let dtype = if info.kind == 0 {
        DType::F32
    } else {
        DType::BF16
    };
    Ok(
        Tensor::from_raw_buffer(&index.data(reader, name)?, dtype, &dims, device)?
            .to_dtype(DType::F32)?,
    )
}
