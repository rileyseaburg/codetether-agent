//! Regression: a long agent loop after the last user turn must still fit.

use crate::provider::{ContentPart, Message, Role};
use crate::session::helper::token::estimate_request_tokens;

use super::terminal::terminal_truncate_messages;

fn text(role: Role, body: String) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: body }],
    }
}

#[test]
fn terminal_truncate_evicts_many_small_messages_to_fit() {
    let mut messages = vec![text(Role::User, "do the big task please ".repeat(10))];
    for _ in 0..500 {
        messages.push(text(Role::Assistant, "x".repeat(900)));
    }
    let _ = terminal_truncate_messages(&mut messages, "", &[], 4, 5_000);
    let after = estimate_request_tokens("", &messages, &[]);
    assert!(after <= 5_000, "after={after}");
    assert!(matches!(messages[1].role, Role::User), "user anchor kept");
    assert!(messages.len() >= 2);
}
