//! No active user task and no marker preserve the legacy fallback.
use super::*;
#[test]
fn requests_legacy_when_no_active_task_exists() {
    assert!(matches!(
        prepare(vec![message(Role::Assistant, "old")], 1, 0, |_| 1),
        ResetPreparation::Legacy
    ));
}
