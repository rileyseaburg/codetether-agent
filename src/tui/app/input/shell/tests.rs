//! Tests for the `!command` shell prefix in chat submit.

use crate::tui::app::state::App;
use crate::tui::chat::message::MessageType;

#[tokio::test]
async fn bang_prefix_runs_shell_and_records_output() {
    let mut app = App::default();
    let cwd = std::path::Path::new(".");

    let consumed = super::run(&mut app, cwd, "!echo hello-shell").await;

    assert!(consumed);
    assert!(app.state.input.is_empty());
    let last = app.state.messages.last().expect("tool result message");
    match &last.message_type {
        MessageType::ToolResult {
            name,
            output,
            success,
            ..
        } => {
            assert_eq!(name, "shell");
            assert!(*success);
            assert!(output.contains("hello-shell"));
        }
        other => panic!("expected ToolResult, got {other:?}"),
    }
}

#[tokio::test]
async fn bang_prefix_reports_failure_status() {
    let mut app = App::default();
    let cwd = std::path::Path::new(".");

    assert!(super::run(&mut app, cwd, "!false").await);
    match &app.state.messages.last().expect("message").message_type {
        MessageType::ToolResult { success, .. } => assert!(!*success),
        other => panic!("expected ToolResult, got {other:?}"),
    }
}

#[tokio::test]
async fn non_bang_prompt_is_not_consumed() {
    let mut app = App::default();
    let cwd = std::path::Path::new(".");
    assert!(!super::run(&mut app, cwd, "hello").await);
}

#[tokio::test]
async fn empty_bang_shows_usage() {
    let mut app = App::default();
    let cwd = std::path::Path::new(".");
    assert!(super::run(&mut app, cwd, "!").await);
    assert!(app.state.status.contains("Usage"));
}
