//! Tests for pasted image data URLs.

#[cfg(test)]
mod tests {
    use base64::Engine;

    use crate::tui::app::input::handle_paste;
    use crate::tui::app::state::App;
    use crate::tui::models::ViewMode;

    #[tokio::test]
    async fn image_data_url_paste_attaches_without_inserting_text() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        app.state.input = "draft".to_string();
        app.state.input_cursor = app.state.input.len();
        let payload = base64::engine::general_purpose::STANDARD.encode("png bytes");
        let text = format!("data:image/png;base64,{payload}");

        handle_paste(&mut app, &text).await;

        assert_eq!(app.state.input, "draft");
        assert_eq!(app.state.pending_images.len(), 1);
        assert_eq!(
            app.state.pending_images[0].mime_type.as_deref(),
            Some("image/png")
        );
        assert_eq!(app.state.pending_images[0].data_url, text);
        assert_eq!(
            app.state.status,
            "Attached pasted image. Press Enter to send, or add text first."
        );
    }
}
