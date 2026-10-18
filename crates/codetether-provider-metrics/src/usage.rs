//! Post-response usage recording: telemetry counters + rate-gate window.

use codetether_provider_core::Usage;

/// Record token usage in the global telemetry counters and feed the
/// proactive rate gate so its sliding window stays accurate.
/// # Arguments
/// Receives provider/model identifiers and the final response's usage counters.
/// # Returns
/// Updates shared telemetry before the rolling rate-gate window.
/// # Examples
/// ```no_run
/// codetether_provider_metrics::record_step_usage("provider", "model", &Default::default());
/// ```
pub fn record_step_usage(provider: &str, model: &str, usage: &Usage) {
    codetether_telemetry::TOKEN_USAGE.record_model_usage_with_cache(
        model,
        usage.prompt_tokens as u64,
        usage.completion_tokens as u64,
        usage.cache_read_tokens.unwrap_or(0) as u64,
        usage.cache_write_tokens.unwrap_or(0) as u64,
    );
    codetether_rate_gate::record_usage(provider, model, usage.completion_tokens as u32);
}
