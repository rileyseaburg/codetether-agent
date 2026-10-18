//! Existing route-permission regression.
use crate::routes::match_policy_rule;

/// Verify realtime upgrades require the same execute capability.
#[test]
fn policy_realtime_session_requires_execute_permission() {
    let permission = match_policy_rule("/api/realtime/session/abc123", "GET");
    assert_eq!(permission, Some("agent:execute"));
}
