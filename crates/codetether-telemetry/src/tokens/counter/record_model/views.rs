//! Focused operations for the parent telemetry type.

use super::*;

impl AtomicTokenCounter {
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
