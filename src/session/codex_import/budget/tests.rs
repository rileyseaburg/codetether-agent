//! Tests for the [`BoundedMessages`](super::BoundedMessages) import budget.

use super::*;
use crate::provider::{ContentPart, Role};
use std::path::Path;

fn msg(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}

#[test]
fn retains_only_most_recent_within_cap() {
    let mut buf = BoundedMessages::new();
    let total = MAX_IMPORTED_MESSAGES + 10;
    for i in 0..total {
        buf.push(msg(&i.to_string()));
    }
    assert_eq!(buf.dropped(), 10);
    let kept = buf.finish(Path::new("/tmp/rollout.jsonl"));
    assert_eq!(kept.len(), MAX_IMPORTED_MESSAGES);
    // Oldest 10 evicted; first retained message is "10".
    match &kept[0].content[0] {
        ContentPart::Text { text } => assert_eq!(text, "10"),
        other => panic!("unexpected content: {other:?}"),
    }
}

#[test]
fn no_drop_under_cap() {
    let mut buf = BoundedMessages::new();
    buf.push(msg("a"));
    buf.push(msg("b"));
    assert_eq!(buf.dropped(), 0);
    assert_eq!(buf.finish(Path::new("/tmp/x.jsonl")).len(), 2);
}
