//! A server's own diagnostic timeout still triggers backoff and eviction.

use super::{latency_fixture, scan};
use std::sync::Arc;
use tokio::time::{Duration, Instant, timeout};

#[tokio::test]
async fn failed_language_server_is_evicted_and_backed_off() {
    let root = tempfile::tempdir().unwrap();
    let manager = latency_fixture::manager(root.path(), "silent");
    let old_client = manager.get_client("typescript").await.unwrap();
    let files = vec![
        (root.path().join("first.ts"), "export {};".into()),
        (root.path().join("second.ts"), "export {};".into()),
    ];
    let started = Instant::now();
    let (blocked, warnings) = timeout(
        Duration::from_secs(7),
        scan::run(root.path(), "apply_patch", files.clone(), manager.clone()),
    )
    .await
    .expect("preflight exceeded the shared budget");
    assert!(started.elapsed() < Duration::from_secs(7));
    assert!(blocked.is_none());
    assert_eq!(warnings.len(), 2);
    assert!(warnings[0].contains("LSP diagnostics timeout"));
    assert!(warnings[1].contains("backed off"));
    let (_, repeated) = timeout(
        Duration::from_millis(500),
        scan::run(root.path(), "apply_patch", files, manager.clone()),
    )
    .await
    .expect("retry waited on the failed server again");
    assert!(
        repeated
            .iter()
            .all(|warning| warning.contains("backed off"))
    );
    let replacement = manager.get_client("typescript").await.unwrap();
    assert!(!Arc::ptr_eq(&old_client, &replacement));
    manager.shutdown_all().await;
}
