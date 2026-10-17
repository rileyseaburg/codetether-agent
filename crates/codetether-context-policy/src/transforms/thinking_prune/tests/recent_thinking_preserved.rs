//! recent thinking preserved.
use super::*;

#[test]
pub(super) fn recent_thinking_preserved() {
    let mut msgs = vec![Message {
        role: Role::Assistant,
        content: vec![ContentPart::Thinking {
            text: "x".repeat(5000),
            signature: None,
        }],
    }];
    let stats = prune_thinking(&mut msgs);
    assert_eq!(stats.total_bytes_saved, 0);
}
