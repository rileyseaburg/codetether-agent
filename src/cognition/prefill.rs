//! Chunked prefill for quantized CUDA models.
//!
//! candle's quantized CUDA matmul takes a vector kernel only while
//! `batch * seq <= 8`; above that it switches to `dequantize_matmul`,
//! which materialises the whole weight tensor as f32 and OOMs an 8 GB
//! card on an 8B model. Feeding the prompt in small chunks keeps every
//! forward on the memory-cheap path.

use anyhow::{Result, anyhow};

/// Largest prefill batch that stays on candle's quantized vector kernel.
///
/// Mirrors `max_bm` in `candle_core::quantized::cuda`.
pub const MAX_VEC_KERNEL_BATCH: usize = 8;

/// Split a prompt into forward-pass chunks.
///
/// CUDA uses [`MAX_VEC_KERNEL_BATCH`]-sized chunks to avoid the
/// dequantizing path. Other devices prefill in one pass, which is
/// faster and has no such cliff.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cognition::prefill::chunks;
///
/// // CPU prefills in a single pass.
/// assert_eq!(chunks(20, false), vec![20]);
///
/// // CUDA splits into <=8-token chunks.
/// assert_eq!(chunks(20, true), vec![8, 8, 4]);
///
/// // Short prompts need no splitting.
/// assert_eq!(chunks(5, true), vec![5]);
/// ```
pub fn chunks(prompt_len: usize, is_cuda: bool) -> Vec<usize> {
    if prompt_len == 0 {
        return Vec::new();
    }
    if !is_cuda || prompt_len <= MAX_VEC_KERNEL_BATCH {
        return vec![prompt_len];
    }
    let mut sizes = vec![MAX_VEC_KERNEL_BATCH; prompt_len / MAX_VEC_KERNEL_BATCH];
    let remainder = prompt_len % MAX_VEC_KERNEL_BATCH;
    if remainder > 0 {
        sizes.push(remainder);
    }
    sizes
}

/// Run `forward` over each prefill chunk, returning the final logits.
///
/// `forward` receives a token slice plus the running cache position and
/// must advance the model's KV cache. Keeps the chunk bookkeeping out of
/// the inference loop.
///
/// # Errors
///
/// Propagates the first `forward` error, or errors if `prefill` is empty.
pub fn run<T, F>(prefill: &[u32], index_pos: &mut usize, is_cuda: bool, mut forward: F) -> Result<T>
where
    F: FnMut(&[u32], usize) -> Result<T>,
{
    let mut logits = None;
    let mut offset = 0usize;
    for size in chunks(prefill.len(), is_cuda) {
        logits = Some(forward(&prefill[offset..offset + size], *index_pos)?);
        *index_pos += size;
        offset += size;
    }
    logits.ok_or_else(|| anyhow!("prefill produced no logits"))
}

#[cfg(test)]
#[path = "prefill_tests.rs"]
mod tests;
