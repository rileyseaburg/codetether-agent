//! Synchronization preserves tool call/result rows in provider-message order.
use codetether_chat_model::MessageType;
use codetether_provider_core::{ContentPart, Message, Role};

#[test]
fn synchronization_preserves_tool_entries() {
    let mut view = super::test_view::ProjectionView::default();
    let messages = [
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "Inspect src/main.rs".to_string(),
            }],
        },
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call_1".to_string(),
                name: "read".to_string(),
                arguments: "{\"path\":\"src/main.rs\"}".to_string(),
                thought_signature: None,
            }],
        },
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "call_1".to_string(),
                content: "fn main() {}".to_string(),
            }],
        },
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: "Read complete.".to_string(),
            }],
        },
    ];

    super::sync_messages_from_source(&mut view, &messages, messages.len(), "session", false);

    assert!(matches!(view.messages[0].message_type, MessageType::User));
    assert!(matches!(
        view.messages[1].message_type,
        MessageType::ToolCall { .. }
    ));
    match &view.messages[2].message_type {
        MessageType::ToolResult { name, success, .. } => {
            assert_eq!(name, "read");
            assert!(*success);
        }
        other => panic!("expected tool result message, got {other:?}"),
    }
    assert!(matches!(
        view.messages[3].message_type,
        MessageType::Assistant
    ));
}
