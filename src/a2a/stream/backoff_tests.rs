//! Tests for [`super::Backoff`].

use super::Backoff;
use std::time::Duration;

#[test]
fn delay_within_bounds() {
    let mut b = Backoff::new(Duration::from_millis(100), Duration::from_secs(5));
    for _ in 0..50 {
        let d = b.next_delay();
        assert!(d >= Duration::from_millis(100));
        assert!(d <= Duration::from_secs(5));
    }
}

#[test]
fn reset_returns_to_base() {
    let mut b = Backoff::new(Duration::from_millis(100), Duration::from_secs(5));
    b.next_delay();
    b.reset();
    // next ceil is base*3 so still bounded near base
    assert!(b.next_delay() <= Duration::from_millis(300));
}
