//! CPU reference for tensor-level Hadamard transforms.
use candle_core::{CpuStorage, Layout, Result, Shape};
pub(super) fn forward(
    inverse: bool,
    input: &CpuStorage,
    il: &Layout,
    signs: &CpuStorage,
    sl: &Layout,
) -> Result<(CpuStorage, Shape)> {
    let width = sl.shape().elem_count();
    if width == 0
        || width % 1024 != 0
        || il.dims().last().copied() != Some(width)
        || il.start_offset() != 0
        || sl.start_offset() != 0
        || !il.is_contiguous()
        || !sl.is_contiguous()
    {
        candle_core::bail!("Invalid Hadamard tensor layout");
    }
    let mut values = input.as_slice::<f32>()?.to_vec();
    let signs = signs.as_slice::<f32>()?;
    if values.len() != il.shape().elem_count() || signs.len() != width {
        candle_core::bail!("Hadamard storage size mismatch");
    }
    for row in values.chunks_exact_mut(width) {
        super::hadamard::transform(row, signs, inverse)
            .map_err(|e| candle_core::Error::Msg(e.to_string()))?;
    }
    Ok((CpuStorage::F32(values), il.shape().clone()))
}
