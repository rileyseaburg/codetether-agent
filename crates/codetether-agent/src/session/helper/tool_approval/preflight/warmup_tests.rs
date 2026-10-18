//! A cold server may outlive the UI budget without forcing the next edit cold again.

use super::{latency_fixture, scan};
use std::sync::Arc;
use tokio::time::{Duration, timeout};

#[tokio::test]
async fn interactive_timeout_retains_server_until_warm_checks_succeed() {
    let root = tempfile::tempdir().unwrap();
    let manager = latency_fixture::manager(root.path(), "slow");
    let original_client = manager.get_client("typescript").await.unwrap();
    let files = vec![
        (root.path().join("first.ts"), "export {};".into()),
        (root.path().join("second.ts"), "export {};".into()),
    ];
    let (blocked, warnings) = timeout(
        Duration::from_secs(6),
        scan::run(root.path(), "apply_patch", files.clone(), manager.clone()),
    )
    .await
    .expect("interactive budget was not enforced");
    assert!(blocked.is_none());
    assert_eq!(warnings.len(), 2);
    assert!(warnings[0].contains("server is retained"));
    assert!(warnings[1].contains("still running"));
    let (_, pending) = timeout(
        Duration::from_millis(200),
        scan::run(root.path(), "apply_patch", files.clone(), manager.clone()),
    )
    .await
    .expect("pending warmup blocked another edit");
    assert_eq!(pending.len(), 2);
    latency_fixture::ready(root.path()).await;
    let warmed = manager.get_client("typescript").await.unwrap();
    assert!(Arc::ptr_eq(&original_client, &warmed));
    let (blocked, warnings) = timeout(
        Duration::from_secs(1),
        scan::run(root.path(), "apply_patch", files, manager.clone()),
    )
    .await
    .expect("warm diagnostics stalled");
    assert!(blocked.is_none());
    assert!(warnings.is_empty(), "{warnings:?}");
    manager.shutdown_all().await;
}
