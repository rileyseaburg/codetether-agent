use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use super::send::send_with_retry;

/// A permanent 429 (expired plan) must bail on the first attempt instead of
/// retrying forever and occupying the worker's only task slot.
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

/// A genuinely transient 429 must stop after the bounded attempt budget rather
/// than retrying without limit.
#[tokio::test]
async fn transient_rate_limit_stops_after_max_attempts() {
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
}

/// A transient failure that later succeeds must still return the success body.
#[tokio::test]
async fn transient_rate_limit_recovers_before_budget() {
    let calls = Arc::new(AtomicU32::new(0));
    let seen = Arc::clone(&calls);
    let (body, status) = send_with_retry(|| {
        let seen = Arc::clone(&seen);
        async move {
            let attempt = seen.fetch_add(1, Ordering::SeqCst) + 1;
            if attempt < 2 {
                return Ok((
                    String::from("Too Many Requests"),
                    reqwest::StatusCode::TOO_MANY_REQUESTS,
                ));
            }
            Ok((String::from("{\"ok\":true}"), reqwest::StatusCode::OK))
        }
    })
    .await
    .expect("retry should recover");

    assert_eq!(status, reqwest::StatusCode::OK);
    assert_eq!(body, "{\"ok\":true}");
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}
