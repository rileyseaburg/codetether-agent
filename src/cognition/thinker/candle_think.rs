//! Inference entry points for the Candle runtime.

use anyhow::{Result, anyhow};
use std::time::Instant;

use super::candle_output::{self, Counts};
use super::candle_runtime::CandleThinker;
use super::{ThinkerOutput, candle_prompt};

impl CandleThinker {
    /// Run inference on an already-formatted prompt (no chat template applied).
    ///
    /// # Errors
    ///
    /// Propagates tokenization, prefill, and decode failures.
    #[cfg(feature = "functiongemma")]
    pub(crate) fn think_raw(&mut self, raw_prompt: &str) -> Result<ThinkerOutput> {
        self.think_inner(raw_prompt)
    }

    /// Apply this model's chat template, then run inference.
    ///
    /// # Errors
    ///
    /// Propagates tokenization, prefill, and decode failures.
    pub(crate) fn think(&mut self, system: &str, user: &str) -> Result<ThinkerOutput> {
        let prompt = candle_prompt::format_chat_prompt(&self.architecture, system, user);
        self.think_inner(&prompt)
    }

    fn think_inner(&mut self, prompt: &str) -> Result<ThinkerOutput> {
        let started_at = Instant::now();
        let mut tokens = self.encode_prompt(prompt)?;
        let counts_prompt = tokens.len() as u32;
        let mut sampler = self.new_logits_processor();

        let plan = self.plan_prefix(&tokens)?;
        let prefill = self.prefill(&tokens, plan.index_pos)?;
        let decoded = self.decode(&mut tokens, prefill.logits, prefill.index_pos, &mut sampler)?;

        let text = self
            .tokenizer
            .decode(&decoded.generated, true)
            .map_err(|e| anyhow!("tokenizer decode failed: {e}"))?;
        let counts = Counts {
            prompt: counts_prompt,
            cache_read: plan.cache_read_tokens,
            cache_write: prefill
                .cache_write_tokens
                .saturating_add(decoded.cache_write_tokens),
        };
        self.cached_tokens = tokens;

        tracing::debug!(
            model = %self.model_label,
            latency_ms = started_at.elapsed().as_millis(),
            prompt_tokens = counts.prompt,
            cache_read_tokens = counts.cache_read,
            cache_write_tokens = counts.cache_write,
            "candle thinker generated thought"
        );
        Ok(candle_output::build(
            &self.model_label,
            text,
            decoded,
            counts,
        ))
    }
}
