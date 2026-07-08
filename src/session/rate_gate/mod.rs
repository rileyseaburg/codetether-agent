//! Proactive rate-limit-aware model scheduling.
//!
//! Before each agent step the session loop calls [`check_rate_gate`].
//! If the primary model (e.g. `bedrock/fable`) is near its RPM/TPM
//! budget, the call returns a lighter substitute model to use for this
//! step instead of waiting for a 429.  The primary is restored at the
//! next step once its window has cooled.
//!
//! After every streaming response completes, call [`record_usage`] so
//! the window stays accurate.

mod gate;
mod limits;
mod window;

pub use gate::RateGate;

use once_cell::sync::Lazy;
use std::sync::Arc;

/// Process-wide rate gate singleton.
pub static RATE_GATE: Lazy<Arc<RateGate>> = Lazy::new(|| Arc::new(RateGate::default()));

#[cfg(test)]
mod tests;

/// Substitute models when the primary is throttled, in preference order.
const SUBSTITUTES: &[(&str, &str)] = &[
    ("openai-codex", "gpt-5.5"),
    ("cerebras", "qwen-3-32b"),
    ("zai", "glm-5.1"),
];

/// Check whether `provider/model` has budget for this step.
///
/// Returns `None` → proceed with primary.
/// Returns `Some((sub_provider, sub_model))` → use this instead.
pub fn check_rate_gate(provider: &str, model: &str) -> Option<(String, String)> {
    // Estimate ~2000 tokens per step as a conservative default.
    let est_tokens = 2_000u32;
    let cooldown = RATE_GATE.try_acquire(provider, model, est_tokens)?;
    tracing::info!(
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
pub fn record_usage(provider: &str, model: &str, output_tokens: u32) {
    RATE_GATE.record(provider, model, output_tokens);
}
