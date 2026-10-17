//! Singleton initialization and duplicate-initialization compatibility.

use crate::{AuditCategory, AuditLog, AuditOutcome, audit_log, init_audit_log, try_audit_log};

#[tokio::test]
async fn global_log_initializes_once_and_preserves_original() {
    assert!(try_audit_log().is_none());
    assert!(std::panic::catch_unwind(audit_log).is_err());
    let log = AuditLog::new(256, None);
    init_audit_log(log.clone()).unwrap();
    log.log(
        AuditCategory::Config,
        "init",
        AuditOutcome::Success,
        None,
        None,
    )
    .await;
    assert_eq!(audit_log().count().await, 1);
    assert!(std::ptr::eq(audit_log(), try_audit_log().unwrap()));
    let rejected = init_audit_log(AuditLog::new(512, None)).unwrap_err();
    assert_eq!(
        format!("{rejected:?}"),
        format!("{:?}", AuditLog::default())
    );
    assert_eq!(audit_log().recent(1).await[0].action, "init");
}
