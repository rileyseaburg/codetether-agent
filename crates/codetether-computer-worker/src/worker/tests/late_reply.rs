//! Mocked-local late responses cannot cross a timed-out or cancelled child boundary.
use super::{super::client, ECHO, fixture};
use std::time::Duration;

const LATE: &str = r#"read line; sleep 0.1; printf '%s\n' '{"success":true,"output":"stale","metadata":{}}'; read line"#;

async fn abandoned_exchange(cancel: bool) {
    let mut slot = Some(fixture(LATE).unwrap());
    let exchange = client::run(&mut slot, b"{}", Duration::from_millis(25), || {
        panic!("must use the existing worker")
    });
    if cancel {
        assert!(
            tokio::time::timeout(Duration::from_millis(10), exchange)
                .await
                .is_err()
        );
    } else {
        let result = exchange.await;
        assert_eq!(result.metadata["error_code"], "COMPUTER_USE_WORKER_TIMEOUT");
    }
    assert!(slot.is_none());
    tokio::time::sleep(Duration::from_millis(150)).await;
    let mut starts = 0;
    let result = client::run(&mut slot, b"{}", Duration::from_secs(2), || {
        starts += 1;
        fixture(ECHO)
    })
    .await;
    assert_eq!(starts, 1);
    assert!(result.success);
    assert_eq!(result.output, "kept");
}

#[tokio::test]
async fn timeout_cannot_supply_a_late_reply_to_next_request() {
    abandoned_exchange(false).await;
}

#[tokio::test]
async fn cancellation_cannot_supply_a_late_reply_to_next_request() {
    abandoned_exchange(true).await;
}
