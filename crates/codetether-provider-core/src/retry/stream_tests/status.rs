//! Streaming status retry regressions.

use super::*;

#[tokio::test(start_paused = true)]
async fn transient_status_exhausts_after_five_attempts() {
    let calls = AtomicU32::new(0);
    let start = tokio::time::Instant::now();
    let error = send_response_with_retry(|| {
        calls.fetch_add(1, Ordering::SeqCst);
        async { Ok(response(503, "unavailable")) }
    })
    .await
    .unwrap_err();
    assert_eq!(calls.load(Ordering::SeqCst), 5);
    assert!(error.to_string().contains("after 5 attempts"));
    assert_eq!(start.elapsed(), std::time::Duration::from_secs(30));
}

#[tokio::test]
async fn permanent_rate_limit_bails_immediately() {
    let calls = AtomicU32::new(0);
    let error = send_response_with_retry(|| {
        calls.fetch_add(1, Ordering::SeqCst);
        async { Ok(response(429, "package has expired")) }
    })
    .await
    .unwrap_err();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(error.to_string().contains("Non-retryable provider error"));
}
