//! Existing route-permission regression.
use crate::routes::match_policy_rule;

#[test]
fn policy_proposal_approval_requires_execute_permission() {
    let permission = match_policy_rule("/v1/cognition/proposals/p1/approve", "POST");
    assert_eq!(permission, Some("agent:execute"));
}
