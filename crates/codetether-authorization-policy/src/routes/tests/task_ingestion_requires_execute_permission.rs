//! Existing route-permission regression.
use crate::routes::match_policy_rule;

#[test]
fn task_ingestion_requires_execute_permission() {
    assert_eq!(match_policy_rule("/task", "POST"), Some("agent:execute"));
}
