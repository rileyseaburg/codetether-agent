//! Prompt tokenization and context-window trimming for Candle inference.

use anyhow::{Result, anyhow};

use super::candle_runtime::CandleThinker;

/// Tokens reserved for generation when trimming an over-long prompt.
const RESERVE: usize = 8;

impl CandleThinker {
    /// Tokenize `prompt`, keeping the tail when it exceeds the context window.
    ///
    /// # Errors
    ///
    /// Returns an error when the tokenizer fails or yields no tokens.
    pub(super) fn encode_prompt(&self, prompt: &str) -> Result<Vec<u32>> {
        let encoding = self
            .tokenizer
            .encode(prompt, true)
            .map_err(|e| anyhow!("tokenizer encode failed: {e}"))?;
        let mut tokens = encoding.get_ids().to_vec();
        if tokens.is_empty() {
            return Err(anyhow!("tokenizer produced an empty prompt token set"));
        }
        if self.context_window > RESERVE && tokens.len() >= self.context_window {
            let budget = self.context_window - RESERVE;
            tokens = tokens[tokens.len().saturating_sub(budget)..].to_vec();
        }
        Ok(tokens)
    }
}
