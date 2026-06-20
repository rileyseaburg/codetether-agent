//! Tests for resolving file references from the transcript.

use std::io::Write;

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

use super::resolve::latest_shared_file;

#[test]
fn finds_path_from_read_tool_call() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("hello.rs");
    let mut f = std::fs::File::create(&file).unwrap();
    writeln!(f, "fn main() {{}}").unwrap();

    let mut app = App::default();
    app.state.messages.push(ChatMessage::new(
        MessageType::ToolCall {
            name: "read".to_string(),
            arguments: format!("{{\"path\":\"{}\"}}", file.display()),
        },
        "read",
    ));

    let found = latest_shared_file(&app, dir.path()).expect("path resolved");
    assert_eq!(found, file);
}

#[test]
fn finds_path_from_file_message() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("note.txt");
    std::fs::write(&file, "hi").unwrap();

    let mut app = App::default();
    app.state.messages.push(ChatMessage::new(
        MessageType::File {
            path: file.display().to_string(),
            size: None,
        },
        "shared",
    ));

    let found = latest_shared_file(&app, dir.path()).expect("path resolved");
    assert_eq!(found, file);
}
