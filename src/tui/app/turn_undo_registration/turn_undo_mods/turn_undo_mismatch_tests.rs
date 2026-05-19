#[cfg(test)]
mod tests {
    use crate::provider::{ContentPart, Message, Role};
    use crate::session::pages::PageKind;
    use crate::tui::app::turn_undo_registration::turn_undo_mods::turn_undo::truncate_last_turns;
    use crate::tui::chat::message::{ChatMessage, MessageType};

    fn user() -> Message {
        Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: "u".into() }],
        }
    }

    #[test]
    fn caps_undo_by_smaller_transcript_user_count() {
        let mut messages = vec![user(), user(), user()];
        let mut pages = vec![PageKind::Conversation; messages.len()];
        let mut ui = vec![ChatMessage::new(MessageType::User, "u")];
        let count = truncate_last_turns(&mut messages, &mut pages, &mut ui, 3);
        assert_eq!(count, 1);
        assert_eq!(messages.len(), 2);
        assert!(ui.is_empty());
    }
}
