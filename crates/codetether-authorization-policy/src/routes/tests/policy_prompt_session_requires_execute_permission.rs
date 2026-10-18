//! Existing route-permission regression.
use crate::routes::match_policy_rule;

#[test]
fn policy_prompt_session_requires_execute_permission() {
    let permission = match_policy_rule("/api/session/abc123/prompt", "POST");
    assert_eq!(permission, Some("agent:execute"));
}
