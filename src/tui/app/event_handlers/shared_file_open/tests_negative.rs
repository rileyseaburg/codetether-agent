//! Negative-path tests for file-reference resolution.

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

use super::resolve::latest_shared_file;

#[test]
fn ignores_nonexistent_and_non_file_tools() {
    let dir = tempfile::tempdir().unwrap();
    let mut app = App::default();
    app.state.messages.push(ChatMessage::new(
        MessageType::ToolCall {
            name: "bash".to_string(),
            arguments: "{\"command\":\"ls\"}".to_string(),
        },
        "bash",
    ));
    app.state.messages.push(ChatMessage::new(
        MessageType::ToolCall {
            name: "read".to_string(),
            arguments: "{\"path\":\"/no/such/file\"}".to_string(),
        },
        "read",
    ));
    assert!(latest_shared_file(&app, dir.path()).is_none());
}
