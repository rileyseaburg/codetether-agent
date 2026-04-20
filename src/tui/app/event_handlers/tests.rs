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

    #[tokio::test]
    async fn rapid_enter_after_chars_inserts_newline_not_submit() {
        // Simulates a terminal that strips bracketed-paste markers:
        // the pasted block arrives as a burst of Char events followed
        // by Enter. The burst heuristic must convert the Enter to an
        // in-buffer `\n` so the paste doesn't fan out into N separate
        // chat messages.
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;

        let cwd = std::path::Path::new(".");
        let mut session = crate::session::Session::new().await.expect("session");
        let (event_tx, _) = tokio::sync::mpsc::channel(8);
        let (result_tx, _) = tokio::sync::mpsc::channel(8);

        // Feed 'a' 'b' — the second key stamps last_key_at just before
        // the Enter arrives, so Enter.elapsed() will be ~microseconds.
        for c in ['a', 'b'] {
            handle_event(
                &mut app,
                cwd,
                &mut session,
                &None,
                &None,
                &event_tx,
                &result_tx,
                KeyEvent {
                    code: KeyCode::Char(c),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: crossterm::event::KeyEventState::NONE,
                },
            )
            .await
            .expect("char");
        }
        // Immediate Enter — should be swallowed into the buffer.
        handle_event(
            &mut app,
            cwd,
            &mut session,
            &None,
            &None,
            &event_tx,
            &result_tx,
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: crossterm::event::KeyEventState::NONE,
            },
        )
        .await
        .expect("enter");

        assert_eq!(
            app.state.input, "ab\n",
            "burst Enter should insert newline, not submit"
        );
        assert!(
            app.state.messages.is_empty(),
            "no user message should be emitted yet"
        );
    }

    #[tokio::test]
    async fn slow_enter_submits_as_normal() {
        // Gap > 20ms between last char and Enter → real human submit.
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;

        let cwd = std::path::Path::new(".");
        let mut session = crate::session::Session::new().await.expect("session");
        let (event_tx, _) = tokio::sync::mpsc::channel(8);
        let (result_tx, _) = tokio::sync::mpsc::channel(8);

        handle_event(
            &mut app,
            cwd,
            &mut session,
            &None,
            &None,
            &event_tx,
            &result_tx,
            KeyEvent {
                code: KeyCode::Char('x'),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: crossterm::event::KeyEventState::NONE,
            },
        )
        .await
        .expect("char");

        tokio::time::sleep(std::time::Duration::from_millis(40)).await;

        handle_event(
            &mut app,
            cwd,
            &mut session,
            &None,
            &None,
            &event_tx,
            &result_tx,
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: crossterm::event::KeyEventState::NONE,
            },
        )
        .await
        .expect("enter");

        assert!(
            !app.state.messages.is_empty(),
            "slow Enter should submit the message"
        );
        assert_eq!(app.state.messages[0].content, "x");
    }
}
