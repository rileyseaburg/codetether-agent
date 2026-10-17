use super::{ContentPart, Message, Role, prompt};

#[path = "window/edges.rs"]
mod edges;

#[test]
fn bounds_history_at_complete_user_turns() {
    let rendered = prompt::render_bounded(&messages(), 700);
    assert!(!rendered.contains("obsolete"));
    assert!(rendered.contains("current task"));
    assert!(rendered.contains("<tool_call>"));
    assert!(rendered.contains("<tool_result>"));
    assert!(rendered.len() <= 700);
}

fn messages() -> Vec<Message> {
    vec![
        text(Role::System, "instructions"),
        text(Role::User, &format!("obsolete {}", "x".repeat(900))),
        text(Role::Assistant, "old reply"),
        text(Role::User, "current task"),
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "new-call".into(),
                name: "read".into(),
                arguments: "{}".into(),
                thought_signature: None,
            }],
        },
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "new-call".into(),
                content: "result".into(),
            }],
        },
    ]
}

fn text(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
