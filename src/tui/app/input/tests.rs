//! Unit tests for the input module.
//!
//! Verifies paste behavior, enter submission without a
//! provider, and image-only submissions.

#[cfg(test)]
mod tests {
    use crate::session::ImageAttachment;
    use crate::tui::app::input::pr_command::create_pr_args;
    use crate::tui::app::input::{handle_enter, handle_paste};
    use crate::tui::app::session_runtime::{self, SessionSlot, TuiSessionHandle};
    use crate::tui::app::state::App;
    use crate::tui::chat::message::MessageType;
    use crate::tui::models::ViewMode;
    use crate::worktree::WorktreeInfo;
    use std::path::PathBuf;

    async fn test_slot() -> SessionSlot {
        SessionSlot::new(crate::session::Session::new().await.expect("session"))
    }

    fn test_runtime() -> TuiSessionHandle {
        let (event_tx, _) = tokio::sync::mpsc::channel(8);
        let (notice_tx, _) = tokio::sync::mpsc::channel(8);
        session_runtime::spawn(event_tx, notice_tx)
    }

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
        let mut slot = test_slot().await;
        let runtime = test_runtime();

        handle_enter(&mut app, cwd, &mut slot, &None, &None, &runtime).await;

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
        let mut slot = test_slot().await;
        let runtime = test_runtime();

        handle_enter(&mut app, cwd, &mut slot, &None, &None, &runtime).await;

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

    #[test]
    fn pr_args_pin_the_base_branch() {
        let wt = WorktreeInfo {
            name: "tui_example".to_string(),
            path: PathBuf::from("."),
            branch: "codetether/tui_example".to_string(),
            active: true,
        };

        let args = create_pr_args(&wt, Some("feature/current"), None, "", &[]);

        assert_eq!(args[0], "pr");
        assert_eq!(args[1], "create");
        assert!(
            args.windows(2)
                .any(|pair| pair == ["--base", "feature/current"])
        );
    }
}
