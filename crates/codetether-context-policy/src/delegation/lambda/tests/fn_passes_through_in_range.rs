//! passes through in range policy.
use super::*;

#[test]
pub(super) fn passes_through_in_range() {
    assert!((clamp_lambda(0.95) - 0.95).abs() < 1e-12);
}
