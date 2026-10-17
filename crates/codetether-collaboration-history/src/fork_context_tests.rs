//! Existing user-turn selection regression tests.
use super::select;
use codetether_provider_core::{ContentPart, Message, Role};

fn text(role: Role, value: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: value.into() }],
    }
}

#[test]
fn all_omits_system_and_incomplete_assistant_tail() {
    let source = vec![
        text(Role::System, "root"),
        text(Role::User, "one"),
        text(Role::Assistant, "done"),
        text(Role::User, "two"),
        text(Role::Assistant, "pending tool call"),
    ];
    let inherited = select(&source, super::ForkTurns::All);
    assert_eq!(inherited.len(), 3);
    assert_eq!(inherited[2].role, Role::User);
}

#[test]
fn count_starts_at_requested_recent_user_turn() {
    let source = vec![
        text(Role::User, "one"),
        text(Role::Assistant, "a"),
        text(Role::User, "two"),
        text(Role::Assistant, "b"),
        text(Role::User, "three"),
        text(Role::Assistant, "pending"),
    ];
    let inherited = select(&source, super::ForkTurns::Count(2));
    assert_eq!(inherited.len(), 3);
    assert_eq!(inherited[0].role, Role::User);
}
