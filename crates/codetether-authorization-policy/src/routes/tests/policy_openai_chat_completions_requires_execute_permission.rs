//! Existing route-permission regression.
use crate::routes::match_policy_rule;

#[test]
fn policy_openai_chat_completions_requires_execute_permission() {
    let permission = match_policy_rule("/v1/chat/completions", "POST");
    assert_eq!(permission, Some("agent:execute"));
}
