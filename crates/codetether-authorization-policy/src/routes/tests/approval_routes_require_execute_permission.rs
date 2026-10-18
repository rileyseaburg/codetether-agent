//! Existing route-permission regression.
use crate::routes::match_policy_rule;

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
