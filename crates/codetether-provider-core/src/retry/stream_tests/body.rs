//! Body-classified streaming error regressions.

use super::*;

#[tokio::test(start_paused = true)]
async fn nonretryable_status_with_transient_body_can_recover() {
    let calls = AtomicU32::new(0);
    let result = send_response_with_retry(|| {
        let attempt = calls.fetch_add(1, Ordering::SeqCst);
        async move {
            Ok(if attempt == 0 {
                response(400, "temporarily overloaded")
            } else {
                response(200, "ok")
            })
        }
    })
    .await
    .unwrap();
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    assert_eq!(result.text().await.unwrap(), "ok");
}

#[tokio::test(start_paused = true)]
async fn nonretryable_status_with_transient_body_exhausts_budget() {
    let calls = AtomicU32::new(0);
    let error = send_response_with_retry(|| {
        calls.fetch_add(1, Ordering::SeqCst);
        async { Ok(response(400, "temporarily overloaded")) }
    })
    .await
    .unwrap_err();
    assert_eq!(calls.load(Ordering::SeqCst), 5);
    assert!(error.to_string().contains("after 5 attempts"));
}
