//! Real scheduling is exercised against mock HTTP, without external Vault credentials.
use super::*;

#[tokio::test]
async fn renewable_token_is_maintained_while_clones_exist() {
    let server = fixture::Fixture::new(true, 1, false, 200).await;
    let original = manager(&server).await;
    let cloned = original.clone();
    drop(original);
    wait_for(&server.state.renewals, 2).await;
    assert_eq!(server.state.lookups.load(Ordering::SeqCst), 1);
    drop(cloned);
    tokio::time::sleep(Duration::from_millis(100)).await;
    let count = server.state.renewals.load(Ordering::SeqCst);
    tokio::time::sleep(Duration::from_millis(700)).await;
    assert_eq!(server.state.renewals.load(Ordering::SeqCst), count);
}

#[tokio::test]
async fn transient_renewal_failure_retries_without_logging_token_data() {
    let server = fixture::Fixture::new(true, 1, false, 503).await;
    let _manager = manager(&server).await;
    wait_for(&server.state.renewals, 2).await;
}

#[tokio::test]
async fn replaced_client_restarts_renewal_monitor() {
    let first = fixture::Fixture::new(true, 1, false, 200).await;
    let second = fixture::Fixture::new(true, 1, false, 200).await;
    let manager = manager(&first).await;
    wait_for(&first.state.lookups, 1).await;
    let replacement = super::manager(&second).await;
    *manager.client.write() = replacement.client();
    manager
        .renewal
        .start(std::sync::Arc::downgrade(&manager.client));
    drop(replacement);
    wait_for(&second.state.renewals, 1).await;
}
