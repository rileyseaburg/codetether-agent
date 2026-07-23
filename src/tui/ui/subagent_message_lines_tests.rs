use super::{Source, append};
use crate::provider::{ContentPart, Message, Role};

#[test]
fn collapses_runtime_system_prompts() {
    let messages = vec![Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: "DELIVERABLE STATUS CONTRACT".into(),
        }],
    }];
    let mut rows = Vec::new();
    append(&mut rows, &messages, Source::ManagedAgent);
    let text = rows
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(text.contains("runtime prompt hidden"));
    assert!(!text.contains("DELIVERABLE STATUS CONTRACT"));
}

#[test]
fn remote_transcript_uses_a2a_role_labels() {
    let messages = vec![
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "inspect".into(),
            }],
        },
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "done".into(),
            }],
        },
    ];
    let mut rows = Vec::new();
    append(&mut rows, &messages, Source::A2aPeer);
    let text = rows
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");

    assert!(text.contains("A2A REQUEST"));
    assert!(text.contains("A2A RESPONSE"));
    assert!(!text.lines().any(|line| line == "USER"));
}
