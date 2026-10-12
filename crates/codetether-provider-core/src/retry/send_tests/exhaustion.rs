//! Retry budget regression tests.

use super::*;

/// Transient rate limits exhaust the bounded budget.
#[tokio::test(start_paused = true)]
async fn transient_rate_limit_stops_after_max_attempts() {
    let start = tokio::time::Instant::now();
    let calls = Arc::new(AtomicU32::new(0));
    let seen = Arc::clone(&calls);
    let result = send_with_retry(|| {
        let seen = Arc::clone(&seen);
        async move {
            seen.fetch_add(1, Ordering::SeqCst);
            Ok((
                String::from("Too Many Requests"),
                reqwest::StatusCode::TOO_MANY_REQUESTS,
            ))
        }
    })
    .await;
    assert!(result.is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 5);
    let message = result.unwrap_err().to_string();
    assert!(
        message.contains("after 5 attempts"),
        "unexpected error: {message}"
    );
    assert_eq!(start.elapsed(), std::time::Duration::from_secs(30));
}
