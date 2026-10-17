//! Autoregressive decode loop for Candle inference.

use anyhow::{Context, Result};
use candle_core::Tensor;
use candle_transformers::generation::LogitsProcessor;

use super::candle_decoded::Decoded;
use super::candle_runtime::CandleThinker;
use super::candle_sampling::sample_next_token;

/// Cap the capacity hint so a misconfigured `max_tokens` cannot pre-reserve
/// gigabytes of `u32` up front. The `Vec` still grows on demand.
const MAX_CAPACITY_HINT: usize = 64 * 1024;

impl CandleThinker {
    /// Generate tokens until EOS, `max_tokens`, or the context window is hit.
    ///
    /// `tokens` is extended in place with each accepted token.
    ///
    /// # Errors
    ///
    /// Propagates sampling, tensor, and model forward failures.
    pub(super) fn decode(
        &mut self,
        tokens: &mut Vec<u32>,
        mut logits: Tensor,
        mut index_pos: usize,
        sampler: &mut LogitsProcessor,
    ) -> Result<Decoded> {
        let mut out = Decoded {
            generated: Vec::with_capacity(self.max_tokens.min(MAX_CAPACITY_HINT)),
            finish_reason: "length".to_string(),
            cache_write_tokens: 0,
        };

        for _ in 0..self.max_tokens {
            let next = sample_next_token(sampler, &self.penalized(&logits, tokens)?)?;
            if self.eos_token_ids.contains(&next) {
                out.finish_reason = "stop".to_string();
                break;
            }
            tokens.push(next);
            out.generated.push(next);
            out.cache_write_tokens = out.cache_write_tokens.saturating_add(1);

            if tokens.len() + 1 >= self.context_window {
                break;
            }
            logits = self.step(tokens, index_pos)?;
            index_pos += 1;
        }
        Ok(out)
    }

    /// Run one single-token forward pass at `index_pos`.
    fn step(&mut self, tokens: &[u32], index_pos: usize) -> Result<Tensor> {
        let input = Tensor::new(&tokens[tokens.len() - 1..], &self.device)?
            .unsqueeze(0)
            .context("failed to create candle input tensor")?;
        self.model
            .forward(&input, index_pos)
            .context("candle model forward failed")?
            .squeeze(0)
            .context("failed to squeeze logits batch dimension")
    }
}
