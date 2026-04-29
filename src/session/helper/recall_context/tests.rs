//! Tests for budget-aware message flattening.

use crate::provider::{ContentPart, Message, Role};
use super::flatten::flatten_messages_with_budget;

#[test]
fn respects_token_budget() {
    let msgs: Vec<Message> = (0..100)
        .map(|i| Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: format!("message number {i} with some content"),
            }],
        })
        .collect();
    let (ctx, truncated) = flatten_messages_with_budget(&msgs, 100);
    assert!(truncated);
    assert!(ctx.contains("message number 0"));
}

#[test]
fn skips_thinking_blocks() {
    let messages = vec![Message {
        role: Role::Assistant,
        content: vec![
            ContentPart::Thinking {
                text: "long internal reasoning".into(),
            },
            ContentPart::Text {
                text: "visible answer".into(),
            },
        ],
    }];
    let (ctx, truncated) = flatten_messages_with_budget(&messages, 10_000);
    assert!(!truncated);
    assert!(!ctx.contains("[Thinking]"));
    assert!(ctx.contains("visible answer"));
}
