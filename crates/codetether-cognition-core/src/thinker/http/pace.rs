//! Existing linear retry delay for thinker HTTP attempts.
use std::time::Duration;

/// Sleep before a retry attempt; the first attempt does not wait.
pub(super) async fn backoff(attempt: u32, observer: &impl super::HttpObserver) {
    if attempt > 0 {
        tokio::time::sleep(Duration::from_millis(500 * u64::from(attempt))).await;
        observer.retry(attempt);
    }
}
