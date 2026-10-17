//! Repetition-penalty application for Candle sampling.

use anyhow::{Context, Result};
use candle_core::Tensor;
use candle_transformers::utils::apply_repeat_penalty;

use super::candle_runtime::CandleThinker;

impl CandleThinker {
    /// Apply the repetition penalty over the recent token window.
    ///
    /// # Errors
    ///
    /// Propagates tensor failures from the penalty kernel.
    pub(super) fn penalized(&self, logits: &Tensor, tokens: &[u32]) -> Result<Tensor> {
        if self.repeat_penalty <= 1.0 {
            return Ok(logits.clone());
        }
        let start_at = tokens.len().saturating_sub(self.repeat_last_n);
        apply_repeat_penalty(logits, self.repeat_penalty, &tokens[start_at..])
            .context("failed to apply repeat penalty")
    }
}
