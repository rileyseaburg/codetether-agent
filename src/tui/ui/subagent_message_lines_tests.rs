use super::append;
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
    append(&mut rows, &messages);
    let text = rows
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(text.contains("runtime prompt hidden"));
    assert!(!text.contains("DELIVERABLE STATUS CONTRACT"));
}
