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
    /// A paste large enough to exceed the summarise threshold must NOT
    /// flood the input with the full content. Instead the input gets a
    /// short `[Pasted text #N: …]` placeholder and the full content is
    /// stashed in the sidecar for expansion at submit time.
    #[tokio::test]
    async fn large_paste_is_summarised_into_sidecar() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;

        // 6 lines — over the 5-line threshold.
        let pasted = "alpha\nbeta\ngamma\ndelta\nepsilon\nzeta";
        handle_paste(&mut app, pasted).await;

        // Input shows the placeholder, not the body.
        assert!(
            app.state.input.starts_with("[Pasted text #1:"),
            "input should be a placeholder, got: {:?}",
            app.state.input
        );
        assert!(!app.state.input.contains("epsilon"));

        // Sidecar holds the real content, keyed by id 1.
        assert_eq!(app.state.pending_text_pastes.len(), 1);
        assert_eq!(app.state.pending_text_pastes[0].id, 1);
        assert_eq!(app.state.pending_text_pastes[0].content, pasted);

        // Status surfaces the line / size summary.
        assert!(
            app.state.status.contains("6 lines"),
            "status missing line count: {:?}",
            app.state.status
        );
        assert!(app.state.status.contains("#1"));
    }

    /// On submit, the user-facing chat message keeps the placeholder
    /// (so the chat scroll-back stays compact) but the prompt sent to
    /// the dispatch layer is the expanded form with the full pasted
    /// body so the agent can actually see the content.
    #[tokio::test]
    async fn submit_expands_placeholder_into_agent_prompt() {
        use crate::tui::app::input::pasted_text::expand_paste_placeholders;

        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;

        let pasted = "one\ntwo\nthree\nfour\nfive\nsix";
        handle_paste(&mut app, pasted).await;
        // Type a question after the placeholder.
        app.state.insert_text("\nplease summarise this");

        // What the user sees in chat history.
        let display_prompt = app.state.input.trim().to_string();
        assert!(display_prompt.starts_with("[Pasted text #1:"));
        assert!(display_prompt.ends_with("please summarise this"));
        assert!(!display_prompt.contains("four"));

        // What the agent will receive.
        let agent_prompt =
            expand_paste_placeholders(&display_prompt, &app.state.pending_text_pastes);
        assert!(agent_prompt.contains("--- Begin pasted text #1"));
        assert!(agent_prompt.contains("four\nfive\nsix"));
        assert!(agent_prompt.contains("--- End pasted text #1 ---"));
        assert!(agent_prompt.contains("please summarise this"));
    }

    /// Two consecutive large pastes produce two distinct sidecar
    /// entries with monotonically increasing ids, and both placeholders
    /// land in the input buffer side-by-side.
    #[tokio::test]
    async fn two_large_pastes_get_independent_ids() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;

        handle_paste(&mut app, "a\nb\nc\nd\ne\nf").await;
        handle_paste(&mut app, "g\nh\ni\nj\nk\nl").await;

        assert_eq!(app.state.pending_text_pastes.len(), 2);
        assert_eq!(app.state.pending_text_pastes[0].id, 1);
        assert_eq!(app.state.pending_text_pastes[1].id, 2);
        assert!(app.state.input.contains("[Pasted text #1:"));
        assert!(app.state.input.contains("[Pasted text #2:"));
    }

    /// A short multi-line paste must continue to land inline. This
    /// guards against regressing the existing behaviour that the
    /// summarise threshold was tuned around.
    #[tokio::test]
    async fn short_multiline_paste_still_inlines() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        handle_paste(&mut app, "alpha\nbeta").await;
        assert_eq!(app.state.input, "alpha\nbeta");
        assert!(app.state.pending_text_pastes.is_empty());
    }

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
