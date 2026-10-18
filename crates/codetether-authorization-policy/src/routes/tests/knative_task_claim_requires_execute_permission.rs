//! Existing route-permission regression.
use crate::routes::match_policy_rule;

#[test]
fn knative_task_claim_requires_execute_permission() {
    assert_eq!(
        match_policy_rule("/v1/knative/tasks/t1/claim", "POST"),
        Some("agent:execute")
    );
}
