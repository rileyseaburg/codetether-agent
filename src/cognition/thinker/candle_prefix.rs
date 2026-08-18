//! KV-cache prefix reuse planning for repeated prompt prefixes.

use anyhow::Result;

use super::candle_runtime::CandleThinker;

/// How much of a prompt is already resident in the KV cache.
pub(super) struct PrefixPlan {
    /// Absolute position the next forward pass starts from.
    pub index_pos: usize,
    /// Prompt tokens served from cache.
    pub cache_read_tokens: u32,
}

impl CandleThinker {
    /// Decide whether `tokens` can extend the cached prefix, resetting the KV
    /// cache when it cannot.
    ///
    /// # Errors
    ///
    /// Propagates KV-cache reset failures for architectures lacking support.
    pub(super) fn plan_prefix(&mut self, tokens: &[u32]) -> Result<PrefixPlan> {
        let reusable = self.model.can_extend_cached_prefix()
            && !self.cached_tokens.is_empty()
            && tokens.len() > self.cached_tokens.len()
            && tokens.starts_with(&self.cached_tokens);

        if reusable {
            return Ok(PrefixPlan {
                index_pos: self.cached_tokens.len(),
                cache_read_tokens: self.cached_tokens.len() as u32,
            });
        }
        if !self.cached_tokens.is_empty() {
            self.model.reset_kv_cache_for_new_request()?;
        }
        Ok(PrefixPlan {
            index_pos: 0,
            cache_read_tokens: 0,
        })
    }
}
