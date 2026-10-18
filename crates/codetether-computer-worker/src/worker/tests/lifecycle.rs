//! Local shell subprocesses simulate worker termination and malformed output.
use super::{super::client, fixture};
use std::time::Duration;

#[tokio::test]
async fn failed_exchange_discards_worker_and_never_retries() {
    for (script, code) in [
        ("read line; exit 0", "EXITED"),
        ("read line; kill -KILL $$", "EXITED"),
        ("read line; printf 'not-json\\n'", "INVALID_FRAME"),
        ("read line; printf '{}'", "INVALID_FRAME"),
        ("read line; exec sleep 5", "TIMEOUT"),
    ] {
        let mut slot = None;
        let mut spawned = 0;
        let result = client::run(&mut slot, b"{}", Duration::from_millis(100), || {
            spawned += 1;
            fixture(script)
        })
        .await;
        assert!(!result.success, "{script}");
        assert_eq!(
            result.metadata["error_code"],
            format!("COMPUTER_USE_WORKER_{code}")
        );
        assert_eq!(result.metadata["action_effects_unknown"], true);
        assert_eq!(result.metadata["shadow_state_lost"], true);
        assert_eq!(result.metadata["retry_performed"], false);
        assert!(slot.is_none());
        assert_eq!(spawned, 1);
    }
}

#[tokio::test]
async fn cancellation_discards_worker() {
    let mut slot = None;
    let result = tokio::time::timeout(
        Duration::from_millis(50),
        client::run(&mut slot, b"{}", Duration::from_secs(10), || {
            fixture("read line; exec sleep 5")
        }),
    )
    .await;
    assert!(result.is_err());
    assert!(slot.is_none());
}
