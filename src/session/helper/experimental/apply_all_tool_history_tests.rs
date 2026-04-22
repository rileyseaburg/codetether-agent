use super::apply_all;
use crate::provider::{ContentPart, Message, Role};

fn user(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}

#[test]
fn apply_all_keeps_unique_tool_history_details() {
    let args = r#"{"path":"src/provider/openai.rs","start_line":1,"end_line":200}"#;
    let output = "endpoint=192.168.50.251:8080\n".repeat(400);
    let mut messages = vec![
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call_1".into(),
                name: "read_file".into(),
                arguments: args.into(),
                thought_signature: None,
            }],
        },
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "call_1".into(),
                content: output.clone(),
            }],
        },
    ];
    for i in 0..12 {
        messages.push(user(&format!("filler {i}")));
    }

    let _ = apply_all(&mut messages);

    assert!(matches!(
        &messages[0].content[0],
        ContentPart::ToolCall { arguments, .. } if arguments == args
    ));
    assert!(matches!(
        &messages[1].content[0],
        ContentPart::ToolResult { content, .. } if content == &output
    ));
}
