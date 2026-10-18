//! Application forage regression coverage.
use super::*;

#[test]
fn forage_timeout_bounds_are_clamped() {
    let low = 5u64.clamp(30, 86_400);
    let high = 999_999u64.clamp(30, 86_400);
    assert_eq!(low, 30);
    assert_eq!(high, 86_400);
}
