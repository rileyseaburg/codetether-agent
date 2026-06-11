use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::clipboard_text::copy_text;

pub(super) fn latest(app: &mut App) {
    let Some(text) = latest_assistant(&app.state.messages) else {
        app.state.status = "No assistant reply to copy.".to_string();
        app.state.clear_input();
        return;
    };
    match copy_text(text) {
        Ok(method) => app.state.status = format!("Copied latest reply ({method})."),
        Err(error) => {
            tracing::warn!(error = %error, "Copy latest reply failed");
            app.state.status = "Could not copy latest reply.".to_string();
        }
    }
    app.state.clear_input();
}

fn latest_assistant(messages: &[ChatMessage]) -> Option<&str> {
    messages
        .iter()
        .rev()
        .find_map(|msg| match msg.message_type {
            MessageType::Assistant => {
                let text = msg.content.trim();
                (!text.is_empty()).then_some(text)
            }
            _ => None,
        })
}

#[cfg(test)]
mod tests {
    use super::latest_assistant;
    use crate::tui::chat::message::{ChatMessage, MessageType};

    #[test]
    fn latest_assistant_skips_system_messages() {
        let messages = vec![
            ChatMessage::new(MessageType::Assistant, "first"),
            ChatMessage::new(MessageType::System, "status"),
        ];
        assert_eq!(latest_assistant(&messages), Some("first"));
    }
}
