//! Tests for tool invocation approval previews.

use super::summarize;
use serde_json::json;

#[test]
fn bash_preview_shows_command() {
    let p = summarize("bash", &json!({"command": "ls -la"})).unwrap();
    assert_eq!(p, "run: ls -la");
}

#[test]
fn write_preview_shows_path() {
    let p = summarize("write", &json!({"path": "src/main.rs"})).unwrap();
    assert_eq!(p, "write file: src/main.rs");
}

#[test]
fn unknown_tool_without_fields_is_none() {
    assert!(summarize("mystery", &json!({"x": 1})).is_none());
}
