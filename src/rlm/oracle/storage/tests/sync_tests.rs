//! Tests for bulk sync_pending behavior.

use super::super::manager::OracleTraceStorage;
use super::fixtures::{MAX, MockErr, MockOk, sample};
use std::sync::Arc;

#[tokio::test]
async fn sync_uploads_and_clears() {
    let tmp = tempfile::tempdir().unwrap();
    let ok = OracleTraceStorage::new_for_test(tmp.path().into(), MAX, Some(Arc::new(MockOk)));
    let _ = ok.persist_record(&sample("unverified")).await.unwrap();

    let fail = OracleTraceStorage::new_for_test(tmp.path().into(), MAX, Some(Arc::new(MockErr)));
    let _ = fail.persist_record(&sample("failed")).await.unwrap();

    let stats = ok.sync_pending().await.unwrap();
    assert!(stats.uploaded >= 1);
    assert_eq!(stats.pending_after, 0);
}
