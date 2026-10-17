//! Capacity, eviction, and shared-handle regression tests.

use crate::{AuditCategory, AuditLog, AuditOutcome};

#[tokio::test]
async fn audit_log_evicts_oldest() {
    for capacity in [0, 100, 128, 150] {
        let log = AuditLog::new(capacity, None);
        let clone = log.clone();
        for i in 0..200 {
            clone
                .log(
                    AuditCategory::Api,
                    format!("req-{i}"),
                    AuditOutcome::Success,
                    None,
                    None,
                )
                .await;
        }
        let retained = capacity.max(128);
        assert_eq!(log.count().await, retained);
        let entries = log.recent(usize::MAX).await;
        assert_eq!(entries[0].action, "req-199");
        assert_eq!(
            entries.last().unwrap().action,
            format!("req-{}", 200 - retained)
        );
    }
}
