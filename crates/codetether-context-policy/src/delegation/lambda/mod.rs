//! delegation::lambda contracts and policy operations.
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
pub const LAMBDA_MAX_INCLUSIVE: f64 = 1.0;
mod fn_env_lambda_override;
pub use fn_env_lambda_override::env_lambda_override;
mod fn_clamp_lambda;
pub use fn_clamp_lambda::clamp_lambda;
#[cfg(test)]
mod tests;
