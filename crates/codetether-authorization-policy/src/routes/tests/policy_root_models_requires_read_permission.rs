//! Existing route-permission regression.
use crate::routes::match_policy_rule;

#[test]
fn policy_root_models_requires_read_permission() {
    let permission = match_policy_rule("/models", "GET");
    assert_eq!(permission, Some("agent:read"));
}
