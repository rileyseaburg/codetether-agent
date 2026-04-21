//! Tests for [`reset_helpers`](super) module.

use crate::provider::{ContentPart, Message, Role};

use super::{build_reset_summary_message, last_user_index};

fn text(role: Role, s: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text {
            text: s.to_string(),
        }],
    }
}

#[test]
fn last_user_index_finds_latest_user_turn() {
    let msgs = vec![
        text(Role::System, "sys"),
        text(Role::User, "first"),
        text(Role::Assistant, "reply"),
        text(Role::User, "second"),
        text(Role::Assistant, "reply2"),
    ];
    assert_eq!(last_user_index(&msgs), Some(3));
}

#[test]
fn last_user_index_is_none_without_user_turn() {
    let msgs = vec![text(Role::System, "sys"), text(Role::Assistant, "noop")];
    assert!(last_user_index(&msgs).is_none());
}

#[test]
fn reset_summary_message_carries_expected_markers() {
    let msg = build_reset_summary_message("the summary body");
    assert!(matches!(msg.role, Role::Assistant));
    if let ContentPart::Text { text } = &msg.content[0] {
        assert!(text.starts_with("[CONTEXT RESET]"));
        assert!(text.contains("the summary body"));
        assert!(text.contains("session_recall"));
    } else {
        panic!("expected text content");
    }
}
