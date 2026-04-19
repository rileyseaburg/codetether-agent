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

    // ---------------------------------------------------------------
    // Steering cancel path
    //
    // Guards the regression where mid-stream steering messages were
    // queued but never submitted until the LLM finished on its own.
    // Root cause: `notify_waiters()` silently drops the signal if no
    // waiter is parked, and the spawned provider task often hadn't
    // polled `cancel.notified()` yet. Switching to `notify_one` stores
    // a permit the next `notified().await` consumes immediately.
    // ---------------------------------------------------------------

    use crate::tui::app::input::chat_steer_queue::queue_steering_while_processing;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::Notify;

    /// The raw invariant: `notify_one` before `notified().await` must
    /// still wake the waiter. `notify_waiters` does NOT do this and is
    /// the exact race that broke TUI steering.
    #[tokio::test]
    async fn notify_one_permit_survives_pre_wait_signal() {
        let cancel = Arc::new(Notify::new());
        cancel.notify_one();
        let wait = tokio::time::timeout(Duration::from_millis(50), cancel.notified());
        wait.await
            .expect("notify_one permit should wake a late waiter");
    }

    /// End-to-end: queue a steering prompt while `processing=true` and
    /// assert the installed cancel handle becomes signalled such that a
    /// subsequent `notified().await` resolves.
    #[tokio::test]
    async fn queue_steering_signals_current_turn_cancel() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        app.state.processing = true;
        let cancel = Arc::new(Notify::new());
        app.state.current_turn_cancel = Some(Arc::clone(&cancel));

        queue_steering_while_processing(&mut app, "steer me");

        assert!(
            app.state.current_turn_cancel.is_some(),
            "cancel handle should remain installed after a steering signal"
        );
        assert_eq!(app.state.steering_count(), 1);

        let wait = tokio::time::timeout(Duration::from_millis(100), cancel.notified());
        wait.await
            .expect("queue_steering_while_processing must signal current_turn_cancel");

        assert!(
            app.state.status.contains("Interrupting"),
            "status should reflect the interrupt, got: {}",
            app.state.status
        );
    }

    /// Second steering during the same turn also fires.
    #[tokio::test]
    async fn repeat_steering_re_signals_cancel() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        app.state.processing = true;
        let cancel = Arc::new(Notify::new());
        app.state.current_turn_cancel = Some(Arc::clone(&cancel));

        queue_steering_while_processing(&mut app, "first");
        tokio::time::timeout(Duration::from_millis(50), cancel.notified())
            .await
            .expect("first steering signal");

        queue_steering_while_processing(&mut app, "second");
        tokio::time::timeout(Duration::from_millis(50), cancel.notified())
            .await
            .expect("second steering signal during the same turn");

        assert_eq!(app.state.steering_count(), 2);
    }
}
