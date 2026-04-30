//! Tests for active-tail context preservation.

use crate::provider::{ContentPart, Message, Role};

use super::active_tail::{active_tail_start, active_user_tail_start};

fn msg(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text {
            text: text.to_string(),
        }],
    }
}

#[test]
fn continuation_keeps_previous_substantive_user_turn() {
    let messages = vec![
        msg(Role::User, "upload episode 4 using the episode 3 path"),
        msg(Role::Assistant, "checking files"),
        msg(Role::Tool, "tool output"),
        msg(Role::User, "continue"),
    ];
    assert_eq!(active_tail_start(&messages, 1), 0);
}

#[test]
fn status_keeps_previous_substantive_user_turn() {
    let messages = vec![
        msg(Role::User, "fix the context loss bug"),
        msg(Role::Assistant, "investigating"),
        msg(Role::User, "status?"),
    ];
    assert_eq!(active_user_tail_start(&messages, 1), Some(0));
}

#[test]
fn no_user_falls_back_to_recent_window() {
    let messages = vec![msg(Role::Assistant, "a"), msg(Role::Assistant, "b")];
    assert_eq!(active_tail_start(&messages, 1), 1);
}
