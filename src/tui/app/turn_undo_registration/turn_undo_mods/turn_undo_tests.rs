#[cfg(test)]
mod tests {
    use crate::provider::{ContentPart, Message, Role};
    use crate::session::pages::PageKind;
    use crate::tui::app::turn_undo_registration::turn_undo_mods::turn_undo::truncate_last_turns;
    use crate::tui::chat::message::{ChatMessage, MessageType};

    fn msg(role: Role, text: &str) -> Message {
        Message {
            role,
            content: vec![ContentPart::Text { text: text.into() }],
        }
    }

    fn chat(message_type: MessageType, text: &str) -> ChatMessage {
        ChatMessage::new(message_type, text)
    }

    #[test]
    fn removes_last_turn_from_session_and_tui() {
        let mut messages = vec![msg(Role::User, "u1"), msg(Role::Assistant, "a1")];
        messages.extend([msg(Role::User, "u2"), msg(Role::Assistant, "a2")]);
        let mut pages = vec![PageKind::Conversation; messages.len()];
        let mut ui = vec![chat(MessageType::User, "u1"), chat(MessageType::Assistant, "a1")];
        ui.extend([chat(MessageType::User, "u2"), chat(MessageType::Assistant, "a2")]);
        let count = truncate_last_turns(&mut messages, &mut pages, &mut ui, 1);
        assert_eq!(count, 1);
        assert_eq!(messages.len(), 2);
        assert_eq!(pages.len(), 2);
        assert_eq!(ui.len(), 2);
    }
}
