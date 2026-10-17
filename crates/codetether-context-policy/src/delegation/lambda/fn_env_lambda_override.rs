//! env lambda override policy.
use super::*;

/// Read `CODETETHER_DELEGATION_LAMBDA` and clamp into `[0.9, 1.0)`.
///
/// Returns `None` when the env var is absent or unparseable so the caller
/// can fall back to its persisted [`DelegationConfig::lambda`].
///
/// [`DelegationConfig::lambda`]: crate::delegation::config::DelegationConfig::lambda
pub fn env_lambda_override() -> Option<f64> {
    let raw = std::env::var("CODETETHER_DELEGATION_LAMBDA").ok()?;
    let value: f64 = raw.trim().parse().ok()?;
    Some(clamp_lambda(value))
}
