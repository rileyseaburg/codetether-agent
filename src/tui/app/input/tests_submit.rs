//! Tests for image-only submission.

#[cfg(test)]
mod tests {
    use crate::session::{ImageAttachment, Session};
    use crate::tui::app::input::handle_enter;
    use crate::tui::app::state::App;
    use crate::tui::chat::message::MessageType;
    use crate::tui::models::ViewMode;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn enter_with_pending_image_sends_even_without_text() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        app.state.pending_images.push(ImageAttachment {
            data_url: "data:image/png;base64,Zm9v".to_string(),
            mime_type: Some("image/png".to_string()),
        });

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
        assert!(matches!(
            app.state.messages.get(1).map(|m| &m.message_type),
            Some(MessageType::Image { .. })
        ));
        assert!(app.state.pending_images.is_empty());
    }
}
