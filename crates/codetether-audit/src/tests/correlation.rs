//! Generated identity and correlation preservation.

use crate::{AuditCategory, AuditLog, AuditOutcome};

#[tokio::test]
async fn correlation_helper_preserves_all_fields() {
    let log = AuditLog::default();
    for _ in 0..2 {
        log.log_with_correlation(
            AuditCategory::ToolExecution,
            "tool",
            AuditOutcome::Denied,
            Some("operator".into()),
            Some(serde_json::json!({"reason": "policy"})),
            Some("okr".into()),
            Some("run".into()),
            Some("relay".into()),
            Some("session".into()),
        )
        .await;
    }
    let entries = log.recent(2).await;
    assert_ne!(entries[0].id, entries[1].id);
    let entry = &entries[0];
    assert!(uuid::Uuid::parse_str(&entry.id).is_ok());
    assert_eq!(entry.category, AuditCategory::ToolExecution);
    assert_eq!(entry.action, "tool");
    assert_eq!(entry.outcome, AuditOutcome::Denied);
    assert_eq!(entry.principal.as_deref(), Some("operator"));
    assert_eq!(entry.detail.as_ref().unwrap()["reason"], "policy");
    assert_eq!(entry.okr_id.as_deref(), Some("okr"));
    assert_eq!(entry.okr_run_id.as_deref(), Some("run"));
    assert_eq!(entry.relay_id.as_deref(), Some("relay"));
    assert_eq!(entry.session_id.as_deref(), Some("session"));
    assert!(entry.duration_ms.is_none());
}
