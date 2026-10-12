//! Streaming network error regressions.

use super::*;

#[tokio::test(start_paused = true)]
async fn transient_network_error_exhausts_without_added_context() {
    let calls = AtomicU32::new(0);
    let error = send_response_with_retry(|| {
        calls.fetch_add(1, Ordering::SeqCst);
        async { Err(anyhow::anyhow!("connection reset")) }
    })
    .await
    .unwrap_err();
    assert_eq!(calls.load(Ordering::SeqCst), 5);
    assert_eq!(error.to_string(), "connection reset");
}

#[tokio::test]
async fn nonretryable_network_error_stops_immediately() {
    let calls = AtomicU32::new(0);
    let error = send_response_with_retry(|| {
        calls.fetch_add(1, Ordering::SeqCst);
        async { Err(anyhow::anyhow!("invalid configuration")) }
    })
    .await
    .unwrap_err();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(error.to_string(), "invalid configuration");
}
