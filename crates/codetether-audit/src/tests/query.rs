//! Query ordering, category filters, limits, and snapshot isolation.

use crate::{AuditCategory, AuditLog, AuditOutcome};

#[tokio::test]
async fn queries_filter_before_limiting_and_return_owned_snapshots() {
    let log = AuditLog::default();
    assert!(log.recent(1).await.is_empty());
    for (category, action) in [
        (AuditCategory::Api, "first"),
        (AuditCategory::Session, "second"),
        (AuditCategory::Api, "third"),
        (AuditCategory::Session, "fourth"),
    ] {
        log.log(category, action, AuditOutcome::Success, None, None)
            .await;
    }
    assert!(log.recent(0).await.is_empty());
    assert!(log.by_category(AuditCategory::Api, 0).await.is_empty());
    assert!(log.by_category(AuditCategory::Auth, 10).await.is_empty());
    let entries = log.by_category(AuditCategory::Api, 10).await;
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].action, "third");
    assert_eq!(entries[1].action, "first");
    let limited = log.by_category(AuditCategory::Api, 1).await;
    assert_eq!(limited.len(), 1);
    assert_eq!(limited[0].action, "third");
    let mut recent = log.recent(2).await;
    assert_eq!(recent.len(), 2);
    assert_eq!(recent[0].action, "fourth");
    assert_eq!(recent[1].action, "third");
    recent[0].action.clear();
    assert_eq!(log.recent(1).await[0].action, "fourth");
}
