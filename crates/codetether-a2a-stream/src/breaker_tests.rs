//! Regression cases for the parent stream module.

use super::CircuitBreaker;

#[test]
fn opens_at_threshold_and_closes_on_success() {
    let mut b = CircuitBreaker::new(2);
    b.record_failure();
    assert!(!b.is_open());
    b.record_failure();
    assert!(b.is_open());
    assert_eq!(b.failure_count(), 2);
    b.record_success();
    assert!(!b.is_open());
    assert_eq!(b.failure_count(), 0);
}
