//! Candle reference recurrence for one token of Gated Delta Net attention.
//! State is [heads, key_dimension, value_dimension]; no external inference backend.
use candle_core::{Result, Tensor};
pub(super) fn step(
    state: &Tensor,
    q: &Tensor,
    k: &Tensor,
    v: &Tensor,
    log_decay: &Tensor,
    beta: &Tensor,
) -> Result<(Tensor, Tensor)> {
    let (heads, keys, values) = state.dims3()?;
    if q.dims() != [heads, keys]
        || k.dims() != [heads, keys]
        || v.dims() != [heads, values]
        || log_decay.elem_count() != heads
        || beta.elem_count() != heads
    {
        candle_core::bail!("GDN state/activation shape mismatch");
    }
    let decayed = state.broadcast_mul(&log_decay.exp()?.reshape((heads, 1, 1))?)?;
    let predicted = k.unsqueeze(1)?.matmul(&decayed)?.squeeze(1)?;
    let delta = (v - &predicted)?.broadcast_mul(&beta.reshape((heads, 1))?)?;
    let correction = k.unsqueeze(2)?.broadcast_mul(&delta.unsqueeze(1)?)?;
    let next = (&decayed + correction)?;
    let output = q
        .unsqueeze(1)?
        .matmul(&next)?
        .squeeze(1)?
        .affine(1.0 / (keys as f64).sqrt(), 0.0)?;
    Ok((output, next))
}
