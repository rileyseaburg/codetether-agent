use std::collections::HashMap;

use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use crate::tui::app::state::App;
use crate::tui::app::text::truncate_preview;
use crate::tui::chat::message::{ChatMessage, MessageType};

pub fn extract_message_text(content: &[ContentPart]) -> String {
    let mut out = String::new();
    for part in content {
        match part {
            ContentPart::Text { text } | ContentPart::Thinking { text } => {
                push_line(&mut out, text)
            }
            ContentPart::ToolCall {
                name, arguments, ..
            } => push_line(&mut out, &format!("Tool call: {name} {arguments}")),
            ContentPart::ToolResult { content, .. } => push_line(&mut out, content),
            ContentPart::Image { url, .. } => push_line(&mut out, &format!("[image: {url}]")),
            ContentPart::File { path, .. } => push_line(&mut out, &format!("[file: {path}]")),
        }
    }
    out
}

pub fn sync_messages_from_session(app: &mut App, session: &Session) {
    app.state.messages = session_messages_to_chat_messages(session);
    app.state.current_request_first_token_ms = None;
    app.state.current_request_last_token_ms = None;
    app.state.last_request_first_token_ms = None;
    app.state.last_request_last_token_ms = None;
    app.state.last_completion_model = None;
    app.state.last_completion_latency_ms = None;
    app.state.last_completion_prompt_tokens = None;
    app.state.last_completion_output_tokens = None;
    app.state.last_tool_name = None;
    app.state.last_tool_latency_ms = None;
    app.state.last_tool_success = None;
    app.state.chat_latency.clear();
    app.state.reset_tool_preview_scroll();
    app.state.set_tool_preview_max_scroll(0);
    app.state.scroll_to_bottom();
}

fn session_messages_to_chat_messages(session: &Session) -> Vec<ChatMessage> {
    let mut chat_messages = Vec::new();
    let mut tool_call_names = HashMap::new();

    for message in session.history() {
        if is_hidden_context_marker(message) {
            continue;
        }
        chat_messages.extend(chat_messages_from_provider_message(
            message,
            &mut tool_call_names,
        ));
    }

    chat_messages
}

fn is_hidden_context_marker(message: &Message) -> bool {
    matches!(
        (&message.role, message.content.as_slice()),
        (
            Role::Assistant,
            [ContentPart::Text { text }]
        ) if text.starts_with("[AUTO CONTEXT COMPRESSION]")
            || text.starts_with("[CONTEXT TRUNCATED]")
    )
}

fn chat_messages_from_provider_message(
    message: &Message,
    tool_call_names: &mut HashMap<String, String>,
) -> Vec<ChatMessage> {
    let mut chat_messages = Vec::new();
    let mut text_buffer = String::new();

    for part in &message.content {
        match part {
            ContentPart::Text { text } => push_line(&mut text_buffer, text),
            ContentPart::Thinking { text } => {
                flush_text_buffer(message.role, &mut text_buffer, &mut chat_messages);
                chat_messages.push(ChatMessage::new(
                    MessageType::Thinking(text.clone()),
                    truncate_preview(text, 600),
                ));
            }
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => {
                flush_text_buffer(message.role, &mut text_buffer, &mut chat_messages);
                tool_call_names.insert(id.clone(), name.clone());
                chat_messages.push(ChatMessage::new(
                    MessageType::ToolCall {
                        name: name.clone(),
                        arguments: crate::tui::chat::payload::tool_arguments(arguments),
                    },
                    format!("{name}: {}", truncate_preview(arguments, 240)),
                ));
            }
            ContentPart::ToolResult {
                tool_call_id,
                content,
            } => {
                flush_text_buffer(message.role, &mut text_buffer, &mut chat_messages);
                let name = tool_call_names
                    .get(tool_call_id)
                    .cloned()
                    .unwrap_or_else(|| "tool".to_string());
                let success = !content.trim_start().starts_with("Error:");
                chat_messages.push(ChatMessage::new(
                    MessageType::ToolResult {
                        name: name.clone(),
                        output: crate::tui::chat::payload::tool_output(content),
                        success,
                        duration_ms: None,
                    },
                    format!("{name}: {}", truncate_preview(content, 600)),
                ));
            }
            ContentPart::Image { url, .. } => {
                flush_text_buffer(message.role, &mut text_buffer, &mut chat_messages);
                chat_messages.push(ChatMessage::new(
                    MessageType::Image { url: url.clone() },
                    url.clone(),
                ));
            }
            ContentPart::File { path, .. } => {
                flush_text_buffer(message.role, &mut text_buffer, &mut chat_messages);
                chat_messages.push(ChatMessage::new(
                    MessageType::File {
                        path: path.clone(),
                        size: None,
                    },
                    path.clone(),
                ));
            }
        }
    }

    flush_text_buffer(message.role, &mut text_buffer, &mut chat_messages);
    chat_messages
}

fn flush_text_buffer(role: Role, text_buffer: &mut String, chat_messages: &mut Vec<ChatMessage>) {
    if text_buffer.trim().is_empty() {
        text_buffer.clear();
        return;
    }

    chat_messages.push(ChatMessage::new(
        message_type(role),
        std::mem::take(text_buffer),
    ));
}

fn push_line(out: &mut String, text: &str) {
    if !out.is_empty() {
        out.push('\n');
    }
    out.push_str(text);
}

fn message_type(role: Role) -> MessageType {
    match role {
        Role::User => MessageType::User,
        Role::Assistant => MessageType::Assistant,
        Role::System | Role::Tool => MessageType::System,
    }
}

#[cfg(test)]
mod tests {
    use super::{extract_message_text, sync_messages_from_session};
    use crate::provider::{ContentPart, Message, Role};
    use crate::session::Session;
    use crate::tui::app::state::App;
    use crate::tui::chat::message::MessageType;
    use chrono::Utc;

    #[test]
    fn extract_message_text_keeps_tool_markers_for_plain_export() {
        let content = vec![
            ContentPart::Text {
                text: "hello".to_string(),
            },
            ContentPart::ToolCall {
                id: "call_1".to_string(),
                name: "read".to_string(),
                arguments: "{\"path\":\"src/main.rs\"}".to_string(),
                thought_signature: None,
            },
        ];

        assert!(extract_message_text(&content).contains("Tool call: read"));
    }

    #[tokio::test]
    async fn sync_messages_from_session_preserves_tool_entries() {
        let mut app = App::default();
        let mut session = Session::new().await.expect("session should create");
        let now = Utc::now();
        session.created_at = now;
        session.updated_at = now;
        session.messages = vec![
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
