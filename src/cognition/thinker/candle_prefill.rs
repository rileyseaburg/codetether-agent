//! Prompt prefill forward pass for Candle inference.

use anyhow::{Context, Result};
use candle_core::Tensor;

use super::candle_runtime::CandleThinker;

/// Result of prefilling the prompt into the KV cache.
pub(super) struct Prefill {
    /// Logits for the final prompt position, batch dimension removed.
    pub logits: Tensor,
    /// Absolute position after the prefilled tokens.
    pub index_pos: usize,
    /// Tokens written into the KV cache.
    pub cache_write_tokens: u32,
}

impl CandleThinker {
    /// Prefill `tokens` from `index_pos`, resetting the cache if nothing new
    /// remains to prefill.
    ///
    /// # Errors
    ///
    /// Propagates tensor construction and model forward failures.
    pub(super) fn prefill(&mut self, tokens: &[u32], mut index_pos: usize) -> Result<Prefill> {
        if tokens.len() <= index_pos {
            // Exact-token prompt replay leaves nothing to prefill; start fresh.
            self.model.reset_kv_cache_for_new_request()?;
            index_pos = 0;
        }
        let prefill = &tokens[index_pos..];

        let input = Tensor::new(prefill, &self.device)?
            .unsqueeze(0)
            .context("failed to create candle input tensor")?;
        let logits = self
            .model
            .forward(&input, index_pos)
            .context("candle model forward failed")?
            .squeeze(0)
            .context("failed to squeeze logits batch dimension")?;

        Ok(Prefill {
            logits,
            index_pos: index_pos + prefill.len(),
            cache_write_tokens: prefill.len() as u32,
        })
    }
}
