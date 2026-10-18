//! Existing route-permission regression.
use crate::routes::match_policy_rule;

#[test]
fn worker_task_lifecycle_requires_execute_permission() {
    assert_eq!(
        match_policy_rule("/v1/worker/tasks/claim", "POST"),
        Some("agent:execute")
    );
}
