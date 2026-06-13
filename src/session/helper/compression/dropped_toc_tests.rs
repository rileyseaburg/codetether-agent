//! Tests for [`dropped_toc`](super::dropped_toc).

use super::dropped_toc::render_toc;
use crate::provider::{ContentPart, Message, Role};

fn text(role: Role, s: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text {
            text: s.to_string(),
        }],
    }
}

#[test]
fn toc_lists_user_turns_with_base_offset() {
    let prefix = vec![
        text(Role::User, "implement auth middleware"),
        text(Role::Assistant, "done"),
        text(Role::User, "now fix the failing tests"),
    ];
    let toc = render_toc(&prefix, 5);
    assert!(toc.contains("[DROPPED-RANGE INDEX] turns 5-7"));
    assert!(toc.contains("[turn 5] implement auth middleware"));
    assert!(toc.contains("[turn 7] now fix the failing tests"));
    assert!(toc.contains("context_browse"));
}

#[test]
fn toc_empty_when_no_user_text() {
    let prefix = vec![text(Role::Assistant, "only assistant output")];
    assert!(render_toc(&prefix, 0).is_empty());
    assert!(render_toc(&[], 0).is_empty());
}

#[test]
fn toc_caps_entries_and_reports_overflow() {
    let prefix: Vec<Message> = (0..15)
        .map(|i| text(Role::User, &format!("request number {i}")))
        .collect();
    let toc = render_toc(&prefix, 0);
    assert!(toc.contains("plus 5 more user turns"));
    assert!(!toc.contains("request number 12"));
}

#[test]
fn toc_truncates_long_first_lines() {
    let long = "x".repeat(200);
    let toc = render_toc(&[text(Role::User, &long)], 0);
    assert!(toc.contains('…'));
}
