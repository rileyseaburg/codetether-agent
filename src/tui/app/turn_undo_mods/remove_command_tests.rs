#[cfg(test)]
mod tests {
    use crate::provider::{ContentPart, Message, Role};
    use crate::session::Session;
    use crate::tui::app::commands::handle_slash_command;
    use crate::tui::app::state::App;
    use crate::tui::chat::message::{ChatMessage, MessageType};

    #[tokio::test]
    async fn slash_remove_without_agent_undoes_last_turn() {
        let mut app = App::default();
        let mut session = Session::new().await.expect("session");
        session.messages.push(Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: "u".into() }],
        });
        app.state.messages.push(ChatMessage::new(MessageType::User, "u"));
        handle_slash_command(&mut app, ".".as_ref(), &mut session, None, "/remove").await;
        assert!(session.messages.is_empty());
    }
}
