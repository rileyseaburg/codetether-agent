//! Malformed records, local-only retention, and mixed sync outcomes.
use super::super::manager::OracleTraceStorage;

use super::fixtures::{MAX, MockOk, sample};

use std::sync::Arc;

#[tokio::test]
async fn local_retention_then_mixed_sync() {
    let dir = tempfile::tempdir().unwrap();
    let local = OracleTraceStorage::new_for_test(dir.path().into(), MAX, None);
    let record = sample("unverified");
    let result = local.persist_record(&record).await.unwrap();
    assert!(!result.uploaded);
    assert_eq!(result.pending_count, 1);
    assert_eq!(result.warning.as_deref(), Some("Remote not configured"));
    tokio::fs::write(dir.path().join("broken.jsonl"), "broken")
        .await
        .unwrap();
    tokio::fs::write(dir.path().join("ignored.txt"), "ignored")
        .await
        .unwrap();
    let pending = local.sync_pending().await.unwrap();
    assert_eq!(pending.retained, 2);
    assert_eq!(pending.pending_after, 2);
    let remote = OracleTraceStorage::new_for_test(dir.path().into(), MAX, Some(Arc::new(MockOk)));
    let synced = remote.sync_pending().await.unwrap();
    assert_eq!(synced.uploaded, 1);
    assert_eq!(synced.failed, 1);
    assert_eq!(synced.retained, 1);
    assert_eq!(synced.pending_after, 1);
    assert!(dir.path().join("broken.jsonl").exists());
    assert!(dir.path().join("ignored.txt").exists());
    assert!(!std::path::Path::new(&result.spooled_path).exists());
}
