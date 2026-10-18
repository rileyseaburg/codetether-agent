//! Capture global token deltas and benchmark cost after Ralph returns.
use codetether_benchmark::pricing::{ModelPricing, calculate_cost};
use codetether_telemetry::{GlobalTokenSnapshot, TOKEN_USAGE};
use std::time::Duration;
pub(super) struct Measurements {
    pub(super) duration: Duration,
    pub(super) tokens_used: u64,
    pub(super) cost_usd: f64,
}
pub(super) fn finish(
    tokens_before: GlobalTokenSnapshot,
    duration: Duration,
    pricing: &Option<ModelPricing>,
) -> Measurements {
    // Snapshot token usage after
    let tokens_after = TOKEN_USAGE.global_snapshot();
    let input_tokens = tokens_after
        .total
        .input
        .saturating_sub(tokens_before.total.input);
    let output_tokens = tokens_after
        .total
        .output
        .saturating_sub(tokens_before.total.output);
    let tokens_used = input_tokens + output_tokens;

    // Calculate cost using real pricing
    let cost_usd = calculate_cost(input_tokens, output_tokens, pricing);

    Measurements {
        duration,
        tokens_used,
        cost_usd,
    }
}
