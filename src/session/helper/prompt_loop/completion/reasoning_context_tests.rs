use super::sanitize;
use crate::provider::{ContentPart, Message, Role};

fn message() -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::Thinking {
            text: String::new(),
            signature: Some(
                "codetether:openai-reasoning:{\"type\":\"reasoning\",\"encrypted_content\":\"x\"}"
                    .into(),
            ),
        }],
    }
}

#[test]
fn keeps_codex_reasoning_only_for_codex_provider() {
    let mut codex = vec![message()];
    let mut anthropic = codex.clone();

    sanitize(&mut codex, "openai-codex");
    sanitize(&mut anthropic, "anthropic");

    assert_eq!(codex[0].content.len(), 1);
    assert!(anthropic[0].content.is_empty());
}
