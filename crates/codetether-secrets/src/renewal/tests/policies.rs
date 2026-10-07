//! Missing lookup permission, non-renewable tokens and terminal denial behavior.
use super::*;

#[tokio::test]
async fn lookup_denied_can_still_renew_through_renew_self() {
    let server = fixture::Fixture::new(true, 1, true, 200).await;
    let _manager = manager(&server).await;
    wait_for(&server.state.renewals, 2).await;
}

#[tokio::test]
async fn denied_or_expired_token_does_not_retry_forever() {
    let server = fixture::Fixture::new(true, 1, true, 403).await;
    let _manager = manager(&server).await;
    wait_for(&server.state.renewals, 1).await;
    tokio::time::sleep(Duration::from_millis(1300)).await;
    assert_eq!(server.state.renewals.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn nonrenewable_and_nonexpiring_tokens_do_not_spawn_renewal_storms() {
    for (renewable, ttl) in [(false, 1), (true, 0)] {
        let server = fixture::Fixture::new(renewable, ttl, false, 200).await;
        let _manager = manager(&server).await;
        wait_for(&server.state.lookups, 1).await;
        tokio::time::sleep(Duration::from_millis(700)).await;
        assert_eq!(server.state.renewals.load(Ordering::SeqCst), 0);
    }
}
