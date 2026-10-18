//! Coverage for bounded mux polling backoff.

use super::{Backoff, CEILING, DEFAULT_BUDGET, FIRST};
use std::time::Duration;

#[test]
fn first_attempt_is_sub_millisecond() {
    assert!(FIRST < Duration::from_millis(1));
    assert!(Backoff::new().remaining());
}

#[tokio::test(start_paused = true)]
async fn delay_doubles_up_to_the_ceiling() {
    let mut backoff = Backoff::new();
    for _ in 0..20 {
        backoff.sleep().await;
    }
    assert_eq!(backoff.delay, CEILING);
}

#[tokio::test(start_paused = true)]
async fn default_budget_is_exhausted_and_stops_the_loop() {
    let mut backoff = Backoff::new();
    let mut attempts = 0;
    while backoff.remaining() {
        backoff.sleep().await;
        attempts += 1;
    }
    assert!(backoff.elapsed >= DEFAULT_BUDGET);
    assert!(attempts > 100, "fast path should allow many attempts");
}

#[tokio::test(start_paused = true)]
async fn explicit_budget_is_honoured() {
    let mut backoff = Backoff::with_budget(Duration::from_secs(10));
    while backoff.remaining() {
        backoff.sleep().await;
    }
    assert!(backoff.elapsed >= Duration::from_secs(10));
}

#[tokio::test(start_paused = true)]
async fn fast_publish_costs_far_less_than_the_old_fixed_tick() {
    let mut backoff = Backoff::new();
    backoff.sleep().await;
    assert!(
        backoff.elapsed < Duration::from_millis(50),
        "first poll must not cost a full legacy 50ms tick"
    );
}
