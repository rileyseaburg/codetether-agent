//! Buffered status regression tests.

use super::*;

#[tokio::test]
async fn nonretryable_status_returns_api_error() {
    let calls = AtomicU32::new(0);
    let result = send_with_retry(|| {
        calls.fetch_add(1, Ordering::SeqCst);
        async { Ok(("invalid request".into(), reqwest::StatusCode::BAD_REQUEST)) }
    })
    .await
    .unwrap_err();
    assert_eq!(
        result.to_string(),
        "API error: 400 Bad Request invalid request"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test(start_paused = true)]
async fn transient_body_retries_nonretryable_status() {
    let calls = AtomicU32::new(0);
    let result = send_with_retry(|| {
        let attempt = calls.fetch_add(1, Ordering::SeqCst);
        async move {
            if attempt == 0 {
                Ok((
                    "temporarily overloaded".into(),
                    reqwest::StatusCode::BAD_REQUEST,
                ))
            } else {
                Ok(("ok".into(), reqwest::StatusCode::OK))
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(result.0, "ok");
    assert_eq!(calls.load(Ordering::SeqCst), 2);
}
