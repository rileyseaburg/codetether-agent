//! Server startup is subject to the same deadline as document diagnostics.

use super::{check, latency_fixture};
use tokio::time::{Duration, Instant};

#[tokio::test]
async fn cold_server_initialization_continues_after_interactive_deadline() {
    let root = tempfile::tempdir().unwrap();
    let manager = latency_fixture::manager(root.path(), "slow_init");
    let result = check::run(
        &manager,
        root.path(),
        &root.path().join("cold.ts"),
        "export {};",
        Instant::now() + Duration::from_millis(150),
    )
    .await;
    assert!(result.unwrap_err().is::<super::budget::Expired>());
    latency_fixture::ready(root.path()).await;
    let result = check::run(
        &manager,
        root.path(),
        &root.path().join("cold.ts"),
        "export {};",
        Instant::now() + Duration::from_secs(1),
    )
    .await;
    assert!(result.is_ok(), "{result:?}");
    manager.shutdown_all().await;
}
