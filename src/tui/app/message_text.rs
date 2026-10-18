//! Native session synchronization with shared transcript projection.
#[path = "message_text/sync.rs"]
mod sync;
pub use codetether_chat_transcript::extract_message_text;
pub(crate) use codetether_chat_transcript::provider_messages_to_chat_messages;
pub use sync::sync_messages_from_session;
pub(crate) use sync::sync_messages_from_source;

#[cfg(test)]
mod tests {
    use super::sync_messages_from_session;
    use crate::provider::{ContentPart, Message, Role};
    use crate::session::Session;
    use crate::tui::app::state::App;
    use crate::tui::chat::message::MessageType;
    use chrono::Utc;

    #[tokio::test]
    async fn sync_messages_from_session_preserves_tool_entries() {
        let mut app = App::default();
        let mut session = Session::new().await.expect("session should create");
        let now = Utc::now();
        session.created_at = now;
        session.updated_at = now;
        session.messages.extend([
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
        ]);

        sync_messages_from_session(&mut app, &session);

        assert!(matches!(
            app.state.messages[0].message_type,
            MessageType::User
        ));
        assert!(matches!(
            app.state.messages[1].message_type,
            MessageType::ToolCall { .. }
        ));
        match &app.state.messages[2].message_type {
            MessageType::ToolResult { name, success, .. } => {
                assert_eq!(name, "read");
                assert!(*success);
            }
            other => panic!("expected tool result message, got {other:?}"),
        }
        assert!(matches!(
            app.state.messages[3].message_type,
            MessageType::Assistant
        ));
    }
}
