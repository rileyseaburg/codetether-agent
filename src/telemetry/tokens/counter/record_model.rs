//! Per-model recording helpers for [`super::AtomicTokenCounter`].
//!
//! Split from `counter.rs` to keep both files under the 50-line limit.

use super::AtomicTokenCounter;

impl AtomicTokenCounter {
    /// Record per-model usage without prompt-cache data.
    pub fn record_model_usage(&self, model: &str, prompt: u64, completion: u64) {
        self.record_model_usage_with_cache(model, prompt, completion, 0, 0);
    }

    /// Record a completion's usage including prompt-cache read/write tokens.
    ///
    /// `prompt` is the *non-cached* input tokens billed at full price.
    /// `cache_read` is billed at 10% of input price; `cache_write` at 125%
    /// on Anthropic / Bedrock. See [`crate::provider::pricing`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::telemetry::AtomicTokenCounter;
    ///
    /// let c = AtomicTokenCounter::new();
    /// c.record_model_usage_with_cache("claude-sonnet-4", 500, 200, 1_000, 0);
    /// assert_eq!(c.cache_usage_for("claude-sonnet-4"), (1_000, 0));
    /// assert_eq!(c.last_prompt_tokens_for("claude-sonnet-4"), Some(1_500));
    /// ```
    pub fn record_model_usage_with_cache(
        &self,
        model: &str,
        prompt: u64,
        completion: u64,
        cache_read: u64,
        cache_write: u64,
    ) {
        tracing::debug!(
            model,
            prompt,
            completion,
            cache_read,
            cache_write,
            "Recording model usage"
        );
        self.record(prompt, completion);

        if let Ok(mut usage) = self.model_usage.try_lock() {
            let entry = usage.entry(model.to_string()).or_insert((0, 0));
            entry.0 += prompt;
            entry.1 += completion;
        }
        if let Ok(mut last) = self.model_last_prompt_tokens.try_lock() {
            // `prompt` is the *current turn's* full context window
            // (all prior messages re-sent to the provider), which is the
            // signal the TUI wants for its context-% badge.
            last.insert(model.to_string(), prompt + cache_read + cache_write);
        }
        if (cache_read > 0 || cache_write > 0)
            && let Ok(mut cache) = self.model_cache_usage.try_lock()
        {
            let entry = cache.entry(model.to_string()).or_insert((0, 0));
            entry.0 += cache_read;
            entry.1 += cache_write;
        }
    }

    /// Cumulative `(cache_read, cache_write)` token counts for a model.
    /// Returns `(0, 0)` if the map is contended or the model is unknown.
    pub fn cache_usage_for(&self, model: &str) -> (u64, u64) {
        self.model_cache_usage
            .try_lock()
            .ok()
            .and_then(|m| m.get(model).copied())
            .unwrap_or((0, 0))
    }

    /// Most recent turn's full prompt token count (including cache) for a model.
    ///
    /// This reflects the *current* in-flight context size (what the next
    /// request will send), not cumulative lifetime tokens. Returns `None`
    /// if no completion has been recorded for the model yet.
    pub fn last_prompt_tokens_for(&self, model: &str) -> Option<u64> {
        self.model_last_prompt_tokens
            .try_lock()
            .ok()
            .and_then(|m| m.get(model).copied())
    }
}
