//! Unavailable JSONL sinks must not prevent in-memory retention.

use crate::{AuditCategory, AuditLog, AuditOutcome};

#[tokio::test]
async fn unavailable_sink_still_records_in_memory() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("missing-parent/audit.jsonl");
    let log = AuditLog::new(128, Some(path.clone()));
    log.log(
        AuditCategory::Api,
        "retained",
        AuditOutcome::Failure,
        None,
        None,
    )
    .await;
    assert!(!path.exists());
    assert_eq!(log.count().await, 1);
    assert_eq!(log.recent(1).await[0].action, "retained");
}
