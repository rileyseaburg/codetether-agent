use crate::encode_jsonl_delta;
use codetether_provider_core::{ContentPart, Message, Role};

#[test]
fn encode_jsonl_delta_is_line_per_message_and_skippable() {
    let msgs = vec![
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "one".to_string(),
            }],
        },
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "two".to_string(),
            }],
        },
    ];

    let full = encode_jsonl_delta(&msgs, 0).unwrap();
    assert_eq!(full.lines().count(), 2);
    assert!(full.contains("\"one\""));
    assert!(full.contains("\"two\""));

    let tail = encode_jsonl_delta(&msgs, 1).unwrap();
    assert_eq!(tail.lines().count(), 1);
    assert!(tail.contains("\"two\""));

    let nothing = encode_jsonl_delta(&msgs, 2).unwrap();
    assert!(nothing.is_empty());
}
