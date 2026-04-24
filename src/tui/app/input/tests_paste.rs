//! Tests for paste behavior in chat input.

#[cfg(test)]
mod tests {
    use crate::session::Session;
    use crate::tui::app::input::{handle_enter, handle_paste};
    use crate::tui::app::state::App;
    use crate::tui::chat::message::MessageType;
    use crate::tui::models::ViewMode;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn paste_keeps_multiline_text_in_single_chat_input() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        handle_paste(&mut app, "first line\nsecond line").await;
        assert_eq!(app.state.input, "first line\nsecond line");
        assert_eq!(app.state.status, "Pasted 2 lines into input");
    }

    /// Pasting a multi-line block and then pressing Enter must produce
    /// exactly ONE user message containing every line — not one message
    /// per line. This is the behavior the user explicitly asked us to
    /// validate: "chunks of text should come as one message".
    #[tokio::test]
    async fn multiline_paste_then_enter_submits_as_single_message() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;

        let pasted = "line one\nline two\nline three";
        handle_paste(&mut app, pasted).await;

        // Pre-condition: the paste landed as one buffer, not split.
        assert_eq!(app.state.input, pasted);

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

        // Count only User messages — there will also be a Provider
        // error message because no provider is configured in this test.
        let user_messages: Vec<&str> = app
            .state
            .messages
            .iter()
            .filter(|m| matches!(m.message_type, MessageType::User))
            .map(|m| m.content.as_str())
            .collect();

        assert_eq!(
            user_messages.len(),
            1,
            "expected exactly one User message from a multi-line paste, got {}: {:?}",
            user_messages.len(),
            user_messages
        );
        assert_eq!(
            user_messages[0], pasted,
            "the single User message must contain every pasted line verbatim"
        );
        assert!(
            user_messages[0].contains('\n'),
            "newlines must be preserved inside the single message"
        );
    }

    /// The inline variant: typed (or pasted) text that already contains
    /// newlines — e.g. after Alt+Enter / Shift+Enter inserts a `\n` —
    /// must also submit as a single message on Enter.
    #[tokio::test]
    async fn typed_newline_then_enter_submits_as_single_message() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        app.state.input = "hello\nworld".to_string();
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

        let user_messages: Vec<&str> = app
            .state
            .messages
            .iter()
            .filter(|m| matches!(m.message_type, MessageType::User))
            .map(|m| m.content.as_str())
            .collect();

        assert_eq!(user_messages.len(), 1, "got: {:?}", user_messages);
        assert_eq!(user_messages[0], "hello\nworld");
    }
}
