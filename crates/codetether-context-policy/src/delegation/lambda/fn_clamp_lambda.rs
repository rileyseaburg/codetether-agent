//! clamp lambda policy.
use super::*;

/// Clamp `raw` into the documented `[0.9, 1.0)` range.
///
/// `1.0` is preserved as-is — that signals "no decay" at the call site,
/// matching the historical [`DEFAULT_LAMBDA`]. Values strictly below `0.9`
/// snap up to `0.9`; values strictly above `1.0` (or non-finite) fall back
/// to the default so misconfiguration cannot cancel the bandit out.
pub fn clamp_lambda(raw: f64) -> f64 {
    if !raw.is_finite() {
        return DEFAULT_LAMBDA;
    }
    if raw <= 0.0 || raw > LAMBDA_MAX_INCLUSIVE {
        return DEFAULT_LAMBDA;
    }
    if raw < LAMBDA_MIN {
        return LAMBDA_MIN;
    }
    raw
}
