//! one passes through above one falls back policy.
use super::*;

#[test]
pub(super) fn one_passes_through_above_one_falls_back() {
    assert!((clamp_lambda(1.0) - 1.0).abs() < 1e-12);
    assert!((clamp_lambda(2.5) - DEFAULT_LAMBDA).abs() < 1e-12);
}
