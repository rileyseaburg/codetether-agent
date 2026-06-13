//! Model-relative threshold resolution for the Reset derive policy.
//!
//! An absolute threshold (32k) compresses too early on 1M-context
//! models and too late on 32k models. This resolves the effective
//! threshold as a *fraction* of the model's context window, with
//! absolute env overrides taking precedence.
//!
//! Precedence (highest first):
//! 1. `CODETETHER_CONTEXT_RESET_THRESHOLD_TOKENS` — absolute override.
//! 2. `CODETETHER_CONTEXT_RESET_THRESHOLD_FRACTION` — fraction of the
//!    model's context window (clamped to `(0, 0.95]`).
//! 3. Persisted per-session threshold, if any.
//! 4. Default fraction (0.5) of the model's context window.

use crate::provider::limits::context_window_for_model;

/// Default fraction of the model context window that triggers a reset.
const DEFAULT_RESET_FRACTION: f64 = 0.5;

/// Maximum permitted fraction — leave headroom for the completion.
const MAX_RESET_FRACTION: f64 = 0.95;

/// Resolve the effective reset threshold in tokens for `model`.
///
/// `persisted` is the session's stored threshold (`None` when the
/// session policy is not Reset or stores zero).
pub(super) fn resolve_for_model(model: &str, persisted: Option<usize>) -> usize {
    if let Some(tokens) = env_usize("CODETETHER_CONTEXT_RESET_THRESHOLD_TOKENS") {
        return tokens;
    }
    let window = context_window_for_model(model);
    if let Some(fraction) = env_fraction("CODETETHER_CONTEXT_RESET_THRESHOLD_FRACTION") {
        return scaled(window, fraction);
    }
    if let Some(tokens) = persisted.filter(|t| *t > 0) {
        return tokens;
    }
    scaled(window, DEFAULT_RESET_FRACTION)
}

fn scaled(window: usize, fraction: f64) -> usize {
    ((window as f64) * fraction).round() as usize
}

fn env_usize(key: &str) -> Option<usize> {
    std::env::var(key)
        .ok()
        .and_then(|v| v.trim().parse::<usize>().ok())
        .filter(|n| *n > 0)
}

fn env_fraction(key: &str) -> Option<f64> {
    std::env::var(key)
        .ok()
        .and_then(|v| v.trim().parse::<f64>().ok())
        .filter(|f| *f > 0.0)
        .map(|f| f.min(MAX_RESET_FRACTION))
}
