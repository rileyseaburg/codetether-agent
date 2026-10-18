//! Existing route-permission regression.
use crate::routes::match_policy_rule;

#[test]
fn policy_create_session_keeps_sessions_write_permission() {
    let permission = match_policy_rule("/api/session", "POST");
    assert_eq!(permission, Some("sessions:write"));
}
