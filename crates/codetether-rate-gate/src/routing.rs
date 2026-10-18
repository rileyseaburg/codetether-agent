//! Select a substitute only when the primary model lacks headroom.
use crate::RATE_GATE;

/// Substitute models when the primary is throttled, in preference order.
const SUBSTITUTES: &[(&str, &str)] = &[
    ("openai-codex", "gpt-5.5"),
    ("cerebras", "qwen-3-32b"),
    ("zai", "glm-5.3"),
];

/// Check whether `provider/model` has budget for this step.
///
/// Returns `None` → proceed with primary.
/// Returns `Some((sub_provider, sub_model))` → use this instead.
/// # Arguments
/// * `provider` — Primary provider name.
/// * `model` — Primary model name.
/// # Returns
/// A substitute provider/model pair if the primary is throttled and one is available.
/// # Examples
/// ```
/// assert!(codetether_rate_gate::check_rate_gate("unknown", "model").is_none());
/// ```
pub fn check_rate_gate(provider: &str, model: &str) -> Option<(String, String)> {
    // Estimate ~2000 tokens per step as a conservative default.
    let est_tokens = 2_000u32;
    let cooldown = RATE_GATE.try_acquire(provider, model, est_tokens)?;
    tracing::info!(target: "codetether_agent::session::rate_gate",
        primary_provider = %provider,
        primary_model = %model,
        cooldown_ms = cooldown.as_millis(),
        "Rate gate: primary model near limit, routing to substitute"
    );
    for &(sp, sm) in SUBSTITUTES {
        if sp == provider && sm == model {
            continue; // skip self
        }
        if RATE_GATE.try_acquire(sp, sm, est_tokens).is_none() {
            return Some((sp.to_string(), sm.to_string()));
        }
    }
    None // all substitutes also throttled — let primary proceed anyway
}

/// Record a completed request so the sliding window stays accurate.
/// # Arguments
/// * `provider` — Provider that completed the request.
/// * `model` — Model that completed the request.
/// * `output_tokens` — Usage to add to the rolling window.
/// # Examples
/// ```
/// codetether_rate_gate::record_usage("unknown", "model", 10);
/// ```
pub fn record_usage(provider: &str, model: &str, output_tokens: u32) {
    RATE_GATE.record(provider, model, output_tokens);
}
