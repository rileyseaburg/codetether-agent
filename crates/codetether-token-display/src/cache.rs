//! Aggregate prompt-cache hit rates.
use codetether_telemetry::TokenUsageSnapshot;
impl crate::TokenDisplay {
    /// Aggregate prompt-cache hit rate across all recorded models.
    ///
    /// Defined as `cache_read / (cache_read + full_price_input)` × 100,
    /// i.e. what fraction of billable input was served from the cache.
    /// Returns `None` when no cache activity has been recorded (which is
    /// the common case for providers that don't support it).
    pub(crate) fn get_cache_hit_rate(&self, model_snapshots: &[TokenUsageSnapshot]) -> Option<f64> {
        let mut full_input: u64 = 0;
        let mut cache_read: u64 = 0;
        for s in model_snapshots {
            full_input += s.prompt_tokens;
            let (cr, _cw) = codetether_telemetry::TOKEN_USAGE.cache_usage_for(&s.name);
            cache_read += cr;
        }
        let denom = full_input + cache_read;
        if cache_read == 0 || denom == 0 {
            return None;
        }
        Some(cache_read as f64 * 100.0 / denom as f64)
    }
}
