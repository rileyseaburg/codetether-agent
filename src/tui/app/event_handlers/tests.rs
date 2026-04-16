//! Unit tests for event handler dispatch.
//!
//! Covers mouse-wheel scrolling interactions with the chat
//! view's follow-latest sentinel value.
//!
//! # Examples
//!
//! ```ignore
//! cargo test --lib tui::app::event_handlers::tests
//! ```

#[cfg(test)]
mod tests {
    use crossterm::event::{
        KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseEvent, MouseEventKind,
    };

    use crate::tui::app::event_handlers::{handle_event, handle_mouse_event};
    use crate::tui::app::state::App;
    use crate::tui::chat::message::MessageType;
    use crate::tui::models::ViewMode;

    #[test]
    fn mouse_wheel_scrolls_chat_from_follow_latest() {
        let mut app = App::default();
        app.state.set_chat_max_scroll(25);
        app.state.scroll_to_bottom();

        handle_mouse_event(
            &mut app,
            MouseEvent {
                kind: MouseEventKind::ScrollUp,
                column: 0,
                row: 0,
                modifiers: KeyModifiers::NONE,
            },
        );

        assert_eq!(app.state.chat_scroll, 22);
    }

    #[tokio::test]
    async fn enter_key_event_dispatches_to_chat_submit() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        app.state.input = "hello from key event".to_string();
        app.state.input_cursor = app.state.input.chars().count();

        let cwd = std::path::Path::new(".");
        let mut session = crate::session::Session::new().await.expect("session");
        let (event_tx, _) = tokio::sync::mpsc::channel(8);
        let (result_tx, _) = tokio::sync::mpsc::channel(8);

        let key = KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        };

        let quit = handle_event(
            &mut app,
            cwd,
            &mut session,
            &None,
            &None,
            &event_tx,
            &result_tx,
            key,
        )
        .await
        .expect("handle_event");

        assert!(!quit, "Enter should not quit");
        assert!(
            matches!(
                app.state.messages.first().map(|m| &m.message_type),
                Some(MessageType::User)
            ),
            "Expected User message but got {:?}",
            app.state.messages.first().map(|m| &m.message_type)
        );
        assert_eq!(app.state.messages[0].content, "hello from key event");
        assert!(
            app.state.input.is_empty(),
            "Input should be cleared after Enter"
        );
    }

    #[tokio::test]
    async fn question_mark_inserts_into_chat_input() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;

        let cwd = std::path::Path::new(".");
        let mut session = crate::session::Session::new().await.expect("session");
        let (event_tx, _) = tokio::sync::mpsc::channel(8);
        let (result_tx, _) = tokio::sync::mpsc::channel(8);

        let key = KeyEvent {
            code: KeyCode::Char('?'),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        };

        let quit = handle_event(
            &mut app,
            cwd,
            &mut session,
            &None,
            &None,
            &event_tx,
            &result_tx,
            key,
        )
        .await
        .expect("handle_event");

        assert!(!quit);
        assert_eq!(
            app.state.input, "?",
            "? should be inserted into chat input, not toggle help"
        );
        assert!(
            !app.state.show_help,
            "Help should not toggle when typing ? in chat"
        );
    }

    #[tokio::test]
    async fn ctrl_s_prefills_steer_command_in_chat() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;

        let cwd = std::path::Path::new(".");
        let mut session = crate::session::Session::new().await.expect("session");
        let (event_tx, _) = tokio::sync::mpsc::channel(8);
        let (result_tx, _) = tokio::sync::mpsc::channel(8);

        let key = KeyEvent {
            code: KeyCode::Char('w'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: crossterm::event::KeyEventState::NONE,
        };

        let quit = handle_event(
            &mut app,
            cwd,
            &mut session,
            &None,
            &None,
            &event_tx,
            &result_tx,
            key,
        )
        .await
        .expect("handle_event");

        assert!(!quit);
        assert_eq!(app.state.input, "/steer ");
    }
}
