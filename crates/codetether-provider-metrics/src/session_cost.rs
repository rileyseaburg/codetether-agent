//! Running session-cost estimate over the shared usage counters.
use codetether_provider_core::pricing::{
    cache_read_multiplier, cache_write_multiplier, pricing_for_model,
};

/// Estimate the running cost (USD) of the current session, using the
/// global [`codetether_telemetry::TOKEN_USAGE`] counters and
/// [`pricing_for_model`]. Applies provider-specific prompt-cache
/// multipliers via [`cache_read_multiplier`] and [`cache_write_multiplier`].
///
/// Assumes `prompt_tokens` in the counter already excludes cached input —
/// which is how [`codetether_telemetry::AtomicTokenCounter::record_model_usage_with_cache`]
/// is invoked from every provider adapter.
pub fn session_cost_usd() -> f64 {
    use codetether_telemetry::TOKEN_USAGE;

    TOKEN_USAGE
        .model_snapshots()
        .iter()
        .map(|s| {
            let (input_price, output_price) = pricing_for_model(&s.name);
            let (cache_read, cache_write) = TOKEN_USAGE.cache_usage_for(&s.name);
            let per_million = |n: u64, price: f64| (n as f64 / 1_000_000.0) * price;
            per_million(s.prompt_tokens, input_price)
                + per_million(s.completion_tokens, output_price)
                + per_million(cache_read, input_price * cache_read_multiplier(&s.name))
                + per_million(cache_write, input_price * cache_write_multiplier(&s.name))
        })
        .sum()
}
