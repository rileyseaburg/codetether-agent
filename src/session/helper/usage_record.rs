//! Post-response usage recording: telemetry counters + rate-gate window.

use crate::provider::Usage;

/// Record token usage in the global telemetry counters and feed the
/// proactive rate gate so its sliding window stays accurate.
pub(crate) fn record_step_usage(provider: &str, model: &str, usage: &Usage) {
    crate::telemetry::TOKEN_USAGE.record_model_usage_with_cache(
        model,
        usage.prompt_tokens as u64,
        usage.completion_tokens as u64,
        usage.cache_read_tokens.unwrap_or(0) as u64,
        usage.cache_write_tokens.unwrap_or(0) as u64,
    );
    crate::session::rate_gate::record_usage(provider, model, usage.completion_tokens as u32);
}
