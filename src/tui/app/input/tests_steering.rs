//! Tests for steering queue consumption on submit.

#[cfg(test)]
mod tests {
    use crate::session::Session;
    use crate::tui::app::input::handle_enter;
    use crate::tui::app::state::App;
    use crate::tui::models::ViewMode;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn queued_steering_is_consumed_on_next_submit() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        app.state.queue_steering("Be concise");
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

        assert_eq!(app.state.messages[0].content, "hello tui");
        assert_eq!(app.state.steering_count(), 0);
        assert!(
            app.state.status.contains("No providers") || app.state.status.contains("Submitting")
        );
    }
}
