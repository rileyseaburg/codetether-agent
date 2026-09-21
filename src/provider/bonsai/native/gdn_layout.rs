//! Convert tiled value-head order to the grouped order required by Prism SSM output.
use candle_core::{Result, Tensor};
pub(super) fn grouped(
    input: &Tensor,
    key_heads: usize,
    repetitions: usize,
    head_width: usize,
) -> Result<Tensor> {
    let expected = key_heads
        .checked_mul(repetitions)
        .and_then(|n| n.checked_mul(head_width))
        .ok_or_else(|| candle_core::Error::Msg("GDN head geometry overflow".into()))?;
    if key_heads == 0
        || repetitions == 0
        || head_width == 0
        || input.dims().last().copied() != Some(expected)
    {
        candle_core::bail!("GDN grouped-head shape mismatch");
    }
    let batches = input.elem_count() / expected;
    input
        .reshape((batches, repetitions, key_heads, head_width))?
        .transpose(1, 2)?
        .contiguous()?
        .reshape(input.shape().clone())
}
