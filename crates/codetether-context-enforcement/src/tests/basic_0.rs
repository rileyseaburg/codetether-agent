use crate::terminal_truncate_messages;
use codetether_provider_core::{ContentPart, Message, Role};

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
