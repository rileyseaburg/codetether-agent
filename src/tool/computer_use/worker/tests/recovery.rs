//! Persistence, result preservation, and lazy restart checks with mock workers.
use super::{super::client, ECHO, fixture};
use std::time::Duration;

#[tokio::test]
async fn success_preserves_result_and_reuses_worker() {
    let mut slot = None;
    let first = client::run(&mut slot, b"{}", Duration::from_secs(2), || fixture(ECHO)).await;
    assert!(first.success);
    assert_eq!(first.output, "kept");
    assert_eq!(first.metadata["remote"], true);
    let second = client::run(&mut slot, b"{}", Duration::from_secs(2), || {
        panic!("must reuse")
    })
    .await;
    assert_eq!(second.output, first.output);
    assert!(slot.is_some());
}

#[tokio::test]
async fn only_next_request_restarts_after_crash() {
    let mut slot = None;
    let failed = client::run(&mut slot, b"{}", Duration::from_secs(2), || {
        fixture("read line; kill -KILL $$")
    })
    .await;
    assert!(!failed.success);
    assert!(slot.is_none());
    let recovered = client::run(&mut slot, b"{}", Duration::from_secs(2), || fixture(ECHO)).await;
    assert!(recovered.success);
    assert!(slot.is_some());
}
