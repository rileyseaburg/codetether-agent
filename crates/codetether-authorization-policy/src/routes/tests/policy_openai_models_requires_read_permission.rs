//! Existing route-permission regression.
use crate::routes::match_policy_rule;

#[test]
fn policy_openai_models_requires_read_permission() {
    let permission = match_policy_rule("/v1/models", "GET");
    assert_eq!(permission, Some("agent:read"));
}
