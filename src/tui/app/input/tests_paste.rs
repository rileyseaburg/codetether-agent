//! Tests for paste behavior in chat input.

#[cfg(test)]
mod tests {
    use crate::tui::app::input::handle_paste;
    use crate::tui::app::state::App;
    use crate::tui::models::ViewMode;

    #[tokio::test]
    async fn paste_keeps_multiline_text_in_single_chat_input() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        handle_paste(&mut app, "first line\nsecond line").await;
        assert_eq!(app.state.input, "first line\nsecond line");
        assert_eq!(app.state.status, "Pasted 2 lines into input");
    }
}
