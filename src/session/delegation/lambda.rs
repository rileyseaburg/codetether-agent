//! Forgetting factor `λ` resolution + range clamp (Phase C step 32).
//!
//! CADMAS-CTX recommends `λ ∈ [0.9, 1.0)` for cyclical-drift mitigation:
//! tighter values forget too quickly, looser values are indistinguishable
//! from no decay. `1.0` (and above) disables decay entirely; values below
//! `0.9` collapse the posterior toward the latest observation.

use super::config::DEFAULT_LAMBDA;

/// Lower bound for the forgetting factor.
pub const LAMBDA_MIN: f64 = 0.9;
/// Upper bound (exclusive): `1.0` would mean "never forget" — handled by
/// the caller, not by this clamp. We saturate at the largest representable
/// value strictly below `1.0`.
pub const LAMBDA_MAX_EXCLUSIVE: f64 = 1.0;

/// Read `CODETETHER_DELEGATION_LAMBDA` and clamp into `[0.9, 1.0)`.
///
/// Returns `None` when the env var is absent or unparseable so the caller
/// can fall back to its persisted [`DelegationConfig::lambda`].
///
/// [`DelegationConfig::lambda`]: super::config::DelegationConfig::lambda
pub fn env_lambda_override() -> Option<f64> {
    let raw = std::env::var("CODETETHER_DELEGATION_LAMBDA").ok()?;
    let value: f64 = raw.trim().parse().ok()?;
    Some(clamp_lambda(value))
}

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
    if raw <= 0.0 || raw > LAMBDA_MAX_EXCLUSIVE {
        return DEFAULT_LAMBDA;
    }
    if raw < LAMBDA_MIN {
        return LAMBDA_MIN;
    }
    raw
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps_below_min_to_min() {
        assert!((clamp_lambda(0.5) - LAMBDA_MIN).abs() < 1e-12);
    }

    #[test]
    fn one_passes_through_above_one_falls_back() {
        assert!((clamp_lambda(1.0) - 1.0).abs() < 1e-12);
        assert!((clamp_lambda(2.5) - DEFAULT_LAMBDA).abs() < 1e-12);
    }

    #[test]
    fn passes_through_in_range() {
        assert!((clamp_lambda(0.95) - 0.95).abs() < 1e-12);
    }

    #[test]
    fn nan_falls_back_to_default() {
        assert!((clamp_lambda(f64::NAN) - DEFAULT_LAMBDA).abs() < 1e-12);
    }
}
