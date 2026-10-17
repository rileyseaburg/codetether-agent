//! nan falls back to default policy.
use super::*;

#[test]
pub(super) fn nan_falls_back_to_default() {
    assert!((clamp_lambda(f64::NAN) - DEFAULT_LAMBDA).abs() < 1e-12);
}
