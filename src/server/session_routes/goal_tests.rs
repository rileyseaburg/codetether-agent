//! Goal routes retain the existing authenticated session policy surface.

/// Reads require session access; goal changes require agent execution rights.
#[test]
fn session_goal_controls_use_session_policy() {
    assert_eq!(
        crate::server::match_policy_rule("/api/session/session-a/goal", "GET"),
        Some("sessions:read")
    );
    assert_eq!(
        crate::server::match_policy_rule("/api/session/session-a/goal", "POST"),
        Some("agent:execute")
    );
}
