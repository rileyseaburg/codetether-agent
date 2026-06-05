//! Tests for image-only submission.

#[cfg(test)]
mod tests {
    use crate::session::ImageAttachment;
    use crate::tui::app::input::handle_enter;
    use crate::tui::app::session_runtime::{self, SessionSlot, TuiSessionHandle};
    use crate::tui::app::state::App;
    use crate::tui::chat::message::MessageType;
    use crate::tui::models::ViewMode;

    async fn test_slot() -> SessionSlot {
        SessionSlot::new(crate::session::Session::new().await.expect("session"))
    }

    fn test_runtime() -> TuiSessionHandle {
        let (event_tx, _) = tokio::sync::mpsc::channel(8);
        let (notice_tx, _) = tokio::sync::mpsc::channel(8);
        session_runtime::spawn(event_tx, notice_tx)
    }

    #[tokio::test]
    async fn enter_with_pending_image_sends_even_without_text() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        app.state.pending_images.push(ImageAttachment {
            data_url: "data:image/png;base64,Zm9v".to_string(),
            mime_type: Some("image/png".to_string()),
        });

        let cwd = std::path::Path::new(".");
        let mut slot = test_slot().await;
        let runtime = test_runtime();

        handle_enter(&mut app, cwd, &mut slot, &None, &None, &runtime).await;

        assert!(matches!(
            app.state.messages.first().map(|m| &m.message_type),
            Some(MessageType::Image { .. })
        ));
        assert_eq!(app.state.messages[0].content, "[image/png image, 3 B]");
        assert!(app.state.pending_images.is_empty());
    }
}
