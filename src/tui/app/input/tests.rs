//! Unit tests for the input module.
//!
//! Verifies paste behavior, enter submission without a
//! provider, and image-only submissions.

#[cfg(test)]
mod tests {
    use crate::session::{ImageAttachment, Session};
    use crate::tui::app::input::pr_command::create_pr_args;
    use crate::tui::app::input::{handle_enter, handle_paste};
    use crate::tui::app::state::App;
    use crate::tui::chat::message::MessageType;
    use crate::tui::models::ViewMode;
    use crate::worktree::WorktreeInfo;
    use std::path::PathBuf;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn paste_keeps_multiline_text_in_single_chat_input() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        handle_paste(&mut app, "first line\nsecond line").await;
        assert_eq!(app.state.input, "first line\nsecond line");
        assert_eq!(app.state.status, "Pasted 2 lines into input");
    }

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

    #[tokio::test]
    async fn enter_while_processing_shows_hint_and_keeps_input() {
        // Anthropic-style: while a turn is streaming, Enter no longer
        // queues the input. It keeps the typed text in the buffer and
        // shows a status hint pointing the user at Ctrl+C or /ask.
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        app.state.processing = true;
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

        // Input is preserved (not dispatched, not cleared)
        assert_eq!(app.state.input, "hello tui");
        // No user message appended to the chat buffer
        assert!(
            app.state.messages.is_empty()
                || !app.state.messages.iter().any(|m| m.content == "hello tui")
        );
        // Status points at Ctrl+C / /ask
        assert!(
            app.state.status.contains("Ctrl+C") || app.state.status.contains("/ask"),
            "status should hint at interrupt / ask, got: {}",
            app.state.status
        );
    }

    #[test]
    fn pr_args_pin_the_base_branch() {
        let wt = WorktreeInfo {
            name: "tui_example".to_string(),
            path: PathBuf::from("."),
            branch: "codetether/tui_example".to_string(),
            active: true,
        };

        let args = create_pr_args(&wt, Some("feature/current"));

        assert_eq!(args[0], "pr");
        assert_eq!(args[1], "create");
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--base", "feature/current"])
        );
    }
}
