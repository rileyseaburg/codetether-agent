//! user thinking untouched.
use super::*;

#[test]
pub(super) fn user_thinking_untouched() {
    // Users can't really emit thinking, but defense-in-depth.
    let mut msgs = vec![Message {
        role: Role::User,
        content: vec![ContentPart::Thinking {
            text: "user-thought".repeat(100),
            signature: None,
        }],
    }];
    for i in 0..KEEP_LAST_MESSAGES + 1 {
        msgs.push(Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: format!("r{i}"),
            }],
        });
    }
    let stats = prune_thinking(&mut msgs);
    assert_eq!(stats.total_bytes_saved, 0);
}
