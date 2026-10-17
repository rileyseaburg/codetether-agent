//! Basic recording and generated event defaults.

use crate::{AuditCategory, AuditLog, AuditOutcome};

#[tokio::test]
async fn audit_log_records_and_retrieves() {
    let log = AuditLog::new(100, None);
    let before = chrono::Utc::now();
    log.log(
        AuditCategory::Api,
        "GET /health",
        AuditOutcome::Success,
        None,
        None,
    )
    .await;
    assert_eq!(log.count().await, 1);
    let entry = log.recent(10).await.remove(0);
    assert_eq!(entry.action, "GET /health");
    assert!(uuid::Uuid::parse_str(&entry.id).is_ok());
    assert!(entry.timestamp >= before && entry.timestamp <= chrono::Utc::now());
    assert!(entry.principal.is_none() && entry.detail.is_none());
    assert!(entry.duration_ms.is_none() && entry.okr_id.is_none());
    assert!(entry.okr_run_id.is_none() && entry.relay_id.is_none());
    assert!(entry.session_id.is_none());
    assert!(format!("{log:?}").contains("max_entries: 128"));
    assert!(format!("{:?}", AuditLog::default()).contains("max_entries: 10000"));
}
