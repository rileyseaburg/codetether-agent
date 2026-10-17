//! clamps below min to min policy.
use super::*;

#[test]
pub(super) fn clamps_below_min_to_min() {
    assert!((clamp_lambda(0.5) - LAMBDA_MIN).abs() < 1e-12);
}
