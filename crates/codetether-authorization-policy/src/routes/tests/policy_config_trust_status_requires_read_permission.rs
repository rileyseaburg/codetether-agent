//! Existing route-permission regression.
use crate::routes::match_policy_rule;

#[test]
fn policy_config_trust_status_requires_read_permission() {
    let permission = match_policy_rule("/api/config/trust-status", "GET");
    assert_eq!(permission, Some("agent:read"));
}
