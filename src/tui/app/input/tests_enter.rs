//! Tests for Enter key submission in chat input.

#[cfg(test)]
mod tests {
    use crate::session::Session;
    use crate::tui::app::input::handle_enter;
    use crate::tui::app::state::App;
    use crate::tui::chat::message::MessageType;
    use crate::tui::models::ViewMode;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn enter_echoes_user_message_before_provider_failure() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        app.state.input = "hello tui".to_string();
        app.state.input_cursor = app.state.input.chars().count();

        let cwd = std::path::Path::new(".");
        let mut session = Session::new().await.expect("session");
        let (event_tx, _) = mpsc::channel(8);
        let (result_tx, _) = mpsc::channel(8);

        handle_enter(
            &mut app,
            cwd,
            &mut session,
            &None,
            &None,
            &event_tx,
            &result_tx,
        )
        .await;

        assert!(matches!(
            app.state.messages.first().map(|m| &m.message_type),
            Some(MessageType::User)
        ));
        assert_eq!(app.state.messages[0].content, "hello tui");
        assert!(matches!(
            app.state.messages.get(1).map(|m| &m.message_type),
            Some(MessageType::Error)
        ));
        assert!(app.state.input.is_empty());
    }
}
