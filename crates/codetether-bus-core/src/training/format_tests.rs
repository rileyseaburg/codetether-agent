//! Tests for agent-message training text formatting.

use super::agent_message_text;

#[test]
fn user_facing_text_has_no_routing_prefix() {
    assert_eq!(agent_message_text("build", "user", "done"), "done");
}

#[test]
fn agent_routing_context_is_retained() {
    assert_eq!(
        agent_message_text("build", "planner", "done"),
        "[build → planner] done"
    );
}
