use crate::provider::{ContentPart, Role};
use crate::session::Session;
use crate::tui::app::state::App;
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
    app.state.messages = session
        .messages
        .iter()
        .filter_map(|msg| {
            let content = extract_message_text(&msg.content);
            if content.trim().is_empty() {
                return None;
            }
            Some(ChatMessage::new(message_type(msg.role), content))
        })
        .collect();
    app.state.scroll_to_bottom();
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
