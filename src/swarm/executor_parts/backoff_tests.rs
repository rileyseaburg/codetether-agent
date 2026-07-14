//! Tests for capped exponential retry delays.

use super::calculate;
use tokio::time::Duration;

#[test]
fn grows_exponentially_and_honors_cap() {
    assert_eq!(calculate(0, 100, 1_000, 2.0), Duration::from_millis(100));
    assert_eq!(calculate(2, 100, 1_000, 2.0), Duration::from_millis(400));
    assert_eq!(calculate(9, 100, 1_000, 2.0), Duration::from_millis(1_000));
}
