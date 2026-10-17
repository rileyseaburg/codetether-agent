//! Shared tensor-layout requirements for CPU and CUDA rotations.
use candle_core::{Layout, Result};
pub(super) fn check(input: &Layout, signs: &Layout) -> Result<(usize, usize)> {
    let width = signs.shape().elem_count();
    let count = input.shape().elem_count();
    if width == 0
        || width % 1024 != 0
        || count == 0
        || input.dims().last().copied() != Some(width)
        || input.start_offset() != 0
        || signs.start_offset() != 0
        || !input.is_contiguous()
        || !signs.is_contiguous()
    {
        candle_core::bail!("Invalid Hadamard tensor layout");
    }
    Ok((width, count))
}
