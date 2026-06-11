use super::match_policy_rule;

#[test]
fn task_ingestion_requires_execute_permission() {
    assert_eq!(match_policy_rule("/task", "POST"), Some("agent:execute"));
}

#[test]
fn knative_task_claim_requires_execute_permission() {
    assert_eq!(
        match_policy_rule("/v1/knative/tasks/t1/claim", "POST"),
        Some("agent:execute")
    );
}

#[test]
fn worker_task_lifecycle_requires_execute_permission() {
    assert_eq!(
        match_policy_rule("/v1/worker/tasks/claim", "POST"),
        Some("agent:execute")
    );
}

#[test]
fn approval_routes_require_execute_permission() {
    assert_eq!(
        match_policy_rule("/v1/tools/approvals/request", "POST"),
        Some("agent:execute")
    );
    assert_eq!(
        match_policy_rule("/v1/tools/approvals/abc/decision", "POST"),
        Some("agent:execute")
    );
}
