//! Successful body classification regressions.

use super::*;

#[tokio::test]
async fn successful_body_is_not_classified() {
    let calls = AtomicU32::new(0);
    let text = "rate limit: subscription has expired; service unavailable";
    let result = send_with_retry(|| {
        calls.fetch_add(1, Ordering::SeqCst);
        async { Ok((text.to_owned(), reqwest::StatusCode::OK)) }
    })
    .await
    .unwrap();
    assert_eq!(result.0, text);
    assert_eq!(calls.load(Ordering::SeqCst), 1);
}
