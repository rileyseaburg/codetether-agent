//! Scalar PQ2_0 reference for numerical tests, not a silent inference fallback.
use super::{matmul::Pq2, pq2};
use candle_core::{CpuStorage, Layout, Result, Shape};
pub(super) fn forward(
    op: &Pq2,
    a: &CpuStorage,
    al: &Layout,
    w: &CpuStorage,
    wl: &Layout,
) -> Result<(CpuStorage, Shape)> {
    let shape = op.output(al, wl)?;
    let input = a.as_slice::<f32>()?;
    let bytes = w.as_slice::<u8>()?;
    let blocks = op.columns / 128;
    let expected = op
        .rows
        .checked_mul(blocks)
        .and_then(|n| n.checked_mul(34))
        .ok_or_else(|| candle_core::Error::Msg("PQ2_0 size overflow".into()))?;
    if bytes.len() != expected || input.len() != al.shape().elem_count() {
        candle_core::bail!("PQ2_0 storage length mismatch");
    }
    let mut output = vec![0f32; shape.elem_count()];
    for (batch, values) in input.chunks_exact(op.columns).enumerate() {
        for row in 0..op.rows {
            let mut sum = 0f32;
            for block in 0..blocks {
                let start = (row * blocks + block) * 34;
                let weights = pq2::decode(&bytes[start..start + 34])
                    .map_err(|e| candle_core::Error::Msg(e.to_string()))?;
                sum += values[block * 128..(block + 1) * 128]
                    .iter()
                    .zip(weights)
                    .map(|(a, w)| a * w)
                    .sum::<f32>();
            }
            output[batch * op.rows + row] = sum;
        }
    }
    Ok((CpuStorage::F32(output), shape))
}
