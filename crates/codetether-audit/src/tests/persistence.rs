//! JSONL append semantics and exact entry preservation.

use crate::{AuditCategory, AuditEntry, AuditLog, AuditOutcome};

#[tokio::test]
async fn persistence_appends_without_changing_recorded_entries() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("audit.jsonl");
    for action in ["first", "second"] {
        let log = AuditLog::new(128, Some(path.clone()));
        log.log(
            AuditCategory::Api,
            action,
            AuditOutcome::Success,
            None,
            None,
        )
        .await;
    }
    let text = std::fs::read_to_string(&path).unwrap();
    assert!(text.ends_with('\n'));
    let entries: Vec<AuditEntry> = text
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].action, "first");
    assert_eq!(entries[1].action, "second");
    let log = AuditLog::new(128, Some(path.clone()));
    let mut entry = entries[0].clone();
    entry.duration_ms = Some(42);
    log.record(entry.clone()).await;
    let recorded = serde_json::to_value(log.recent(1).await.remove(0)).unwrap();
    assert_eq!(recorded, serde_json::to_value(&entry).unwrap());
    let text = std::fs::read_to_string(&path).unwrap();
    assert_eq!(text.lines().count(), 3);
    let persisted: serde_json::Value = serde_json::from_str(text.lines().last().unwrap()).unwrap();
    assert_eq!(persisted, recorded);
}
