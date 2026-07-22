use super::{GeminiWebProvider, prompt};
use crate::provider::{ContentPart, Message, Role};
use serde_json::json;

#[path = "prompt_tests/window.rs"]
mod window_tests;

#[test]
fn renders_history_as_parseable_tool_protocol() {
    let messages = vec![
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call-1".into(),
                name: "write".into(),
                arguments: r#"{"path":"hello.sh","content":"echo hi"}"#.into(),
                thought_signature: None,
            }],
        },
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "call-1".into(),
                content: "created".into(),
            }],
        },
    ];

    let rendered = prompt::render(&messages);
    let (_, calls) = GeminiWebProvider::extract_tool_calls(&rendered);

    assert!(!rendered.contains("[Called tool:"));
    assert!(!rendered.contains("[Tool result]"));
    assert!(rendered.contains("<tool_result>"));
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].0, "write");
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&calls[0].1).unwrap(),
        json!({"path": "hello.sh", "content": "echo hi"})
    );
}
