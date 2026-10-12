//! Permanent error regression tests.

use super::*;

/// An expired plan stops on the first attempt.
#[tokio::test]
async fn permanent_rate_limit_body_bails_immediately() {
    let calls = Arc::new(AtomicU32::new(0));
    let seen = Arc::clone(&calls);
    let result = send_with_retry(|| {
        let seen = Arc::clone(&seen);
        async move {
            seen.fetch_add(1, Ordering::SeqCst);
            Ok((
                String::from(r#"{"error":{"code":"1309","message":"package has expired"}}"#),
                reqwest::StatusCode::TOO_MANY_REQUESTS,
            ))
        }
    })
    .await;
    assert!(result.is_err());
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    let message = result.unwrap_err().to_string();
    assert!(
        message.contains("Non-retryable provider error"),
        "unexpected error: {message}"
    );
}
