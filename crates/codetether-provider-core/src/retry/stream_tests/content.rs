//! Successful streaming body regressions.

use super::*;

#[tokio::test]
async fn successful_body_is_returned_unread() {
    let calls = AtomicU32::new(0);
    let body = "rate limit: subscription has expired; service unavailable";
    let result = send_response_with_retry(|| {
        calls.fetch_add(1, Ordering::SeqCst);
        async { Ok(response(200, body)) }
    })
    .await
    .unwrap();
    assert_eq!(result.text().await.unwrap(), body);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn nonretryable_response_stops_immediately() {
    let calls = AtomicU32::new(0);
    let error = send_response_with_retry(|| {
        calls.fetch_add(1, Ordering::SeqCst);
        async { Ok(response(400, "invalid request")) }
    })
    .await
    .unwrap_err();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        error.to_string(),
        "Streaming error: 400 Bad Request invalid request"
    );
}
