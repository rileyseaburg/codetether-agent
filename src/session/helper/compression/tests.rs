//! Tests for the compression module split.

use crate::provider::{ContentPart, Message, Role};

use super::terminal::terminal_truncate_messages;

fn user(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: text.to_string(),
        }],
    }
}

#[test]
fn terminal_truncate_noop_when_short_enough() {
    let mut messages = vec![user("a"), user("b")];
    let before_len = messages.len();
    let dropped = terminal_truncate_messages(&mut messages, "", &[], 4, usize::MAX);
    assert_eq!(dropped, 0);
    assert_eq!(messages.len(), before_len);
}

#[test]
fn terminal_truncate_drops_prefix_and_prepends_marker() {
    let mut messages: Vec<Message> = (0..10).map(|i| user(&format!("msg-{i}"))).collect();
    let _ = terminal_truncate_messages(&mut messages, "", &[], 3, usize::MAX);

    // 1 synthetic marker + 3 most-recent messages.
    assert_eq!(messages.len(), 4);

    // First message is the [CONTEXT TRUNCATED] assistant marker.
    assert!(matches!(messages[0].role, Role::Assistant));
    if let ContentPart::Text { text } = &messages[0].content[0] {
        assert!(text.starts_with("[CONTEXT TRUNCATED]"));
    } else {
        panic!("expected text content on the synthetic marker");
    }

    // Remaining three messages are the most-recent user messages,
    // preserved verbatim and in their original order.
    let kept_texts: Vec<&str> = messages[1..]
        .iter()
        .map(|m| match &m.content[0] {
            ContentPart::Text { text } => text.as_str(),
            _ => panic!("expected text content"),
        })
        .collect();
    assert_eq!(kept_texts, vec!["msg-7", "msg-8", "msg-9"]);
}
