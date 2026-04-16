//! Tests for single-record persist operations.

use super::super::manager::OracleTraceStorage;
use super::fixtures::{MAX, MockErr, MockOk, sample};
use std::path::Path;
use std::sync::Arc;

#[tokio::test]
async fn keeps_spool_when_remote_fails() {
    let tmp = tempfile::tempdir().unwrap();
    let s = OracleTraceStorage::new_for_test(tmp.path().into(), MAX, Some(Arc::new(MockErr)));
    let r = s.persist_record(&sample("failed")).await.unwrap();
    assert!(!r.uploaded);
    assert_eq!(r.pending_count, 1);
    assert!(Path::new(&r.spooled_path).exists());
}

#[tokio::test]
async fn removes_spool_when_remote_succeeds() {
    let tmp = tempfile::tempdir().unwrap();
    let s = OracleTraceStorage::new_for_test(tmp.path().into(), MAX, Some(Arc::new(MockOk)));
    let r = s.persist_record(&sample("golden")).await.unwrap();
    assert!(r.uploaded);
    assert_eq!(r.pending_count, 0);
    assert!(!Path::new(&r.spooled_path).exists());
}
