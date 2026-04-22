use super::apply_all;
use crate::provider::{ContentPart, Message, Role};

fn user(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}

fn assistant(text: &str) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}

#[test]
fn apply_all_keeps_moderate_plaintext_history_intact() {
    let mut messages = vec![
        user("scan lm studio"),
        assistant("confirmed endpoint 192.168.50.251:8080"),
    ];
    for i in 0..24 {
        messages.push(user(&format!("follow-up {i}")));
        messages.push(assistant(&format!("answer {i}")));
    }

    let before_len = messages.len();
    let stats = apply_all(&mut messages);

    assert_eq!(messages.len(), before_len);
    assert_eq!(stats.dedup_hits, 0);
    assert!(messages.iter().any(|message| {
        message.content.iter().any(|part| {
            matches!(
                part,
                ContentPart::Text { text } if text.contains("192.168.50.251:8080")
            )
        })
    }));
}
