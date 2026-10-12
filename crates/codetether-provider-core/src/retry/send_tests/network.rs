//! Buffered network error regression tests.

use super::*;

#[tokio::test(start_paused = true)]
async fn buffered_network_error_exhausts_with_context() {
    let calls = AtomicU32::new(0);
    let error = send_with_retry(|| {
        calls.fetch_add(1, Ordering::SeqCst);
        async { Err(anyhow::anyhow!("connection reset")) }
    })
    .await
    .unwrap_err();
    assert_eq!(calls.load(Ordering::SeqCst), 5);
    assert_eq!(
        error.to_string(),
        "network error persisted after 5 attempts"
    );
}

#[tokio::test]
async fn permanent_network_error_stops_immediately() {
    let calls = AtomicU32::new(0);
    let error = send_with_retry(|| {
        calls.fetch_add(1, Ordering::SeqCst);
        async { Err(anyhow::anyhow!("rate limit: subscription has expired")) }
    })
    .await
    .unwrap_err();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(error.to_string(), "rate limit: subscription has expired");
}

#[tokio::test]
async fn nonretryable_network_error_stops_immediately() {
    let calls = AtomicU32::new(0);
    let error = send_with_retry(|| {
        calls.fetch_add(1, Ordering::SeqCst);
        async { Err(anyhow::anyhow!("invalid configuration")) }
    })
    .await
    .unwrap_err();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert_eq!(error.to_string(), "invalid configuration");
}
