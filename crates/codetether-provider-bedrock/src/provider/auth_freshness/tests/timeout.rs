//! Deadline, retry-window and cancellation checks without wall-clock waiting.
use super::*;
use std::future::pending;

#[tokio::test(start_paused = true)]
async fn timeout_retains_a_bounded_retry_window() {
    let gate = AuthFreshness::default();
    let start = Instant::now();
    assert!(gate.check(pending()).await);
    assert_eq!(start.elapsed(), CHECK_TIMEOUT);
    let reads = AtomicUsize::new(0);
    assert!(!counted(&gate, &reads).await);
    assert_eq!(reads.load(Ordering::SeqCst), 0);
    advance(TTL).await;
    assert!(!counted(&gate, &reads).await);
    assert_eq!(reads.load(Ordering::SeqCst), 1);
}

#[tokio::test(start_paused = true)]
async fn lock_contention_is_also_bounded_by_two_seconds() {
    let gate = AuthFreshness::default();
    let held = gate.0.lock().await;
    let start = Instant::now();
    let reads = AtomicUsize::new(0);
    assert!(counted(&gate, &reads).await);
    assert_eq!(start.elapsed(), CHECK_TIMEOUT);
    assert_eq!(reads.load(Ordering::SeqCst), 0);
    drop(held);
    assert!(!counted(&gate, &reads).await);
    assert_eq!(reads.load(Ordering::SeqCst), 1);
}

#[tokio::test(start_paused = true)]
async fn cancelled_check_releases_single_flight_without_marking_fresh() {
    let gate = AuthFreshness::default();
    let mut check = Box::pin(gate.check(pending()));
    assert!(futures::poll!(check.as_mut()).is_pending());
    drop(check);
    let reads = AtomicUsize::new(0);
    assert!(!counted(&gate, &reads).await);
    assert_eq!(reads.load(Ordering::SeqCst), 1);
}
