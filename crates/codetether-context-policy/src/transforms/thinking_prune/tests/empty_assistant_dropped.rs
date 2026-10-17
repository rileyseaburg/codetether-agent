//! empty assistant dropped.
use super::*;

#[test]
pub(super) fn empty_assistant_dropped() {
    let mut msgs = vec![Message {
        role: Role::Assistant,
        content: vec![ContentPart::Thinking {
            text: "x".repeat(1000),
            signature: None,
        }],
    }];
    for i in 0..KEEP_LAST_MESSAGES + 1 {
        msgs.push(Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: format!("q{i}"),
            }],
        });
    }
    let before = msgs.len();
    prune_thinking(&mut msgs);
    assert_eq!(msgs.len(), before - 1);
}
