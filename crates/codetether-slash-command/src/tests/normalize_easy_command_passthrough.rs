//! Regression coverage retained from native command parsing.
use crate::normalize_easy_command;

#[test]
fn normalize_easy_command_passthrough() {
    assert_eq!(normalize_easy_command("/help"), "/help");
    assert_eq!(normalize_easy_command("/spawn coder"), "/spawn coder");
    assert_eq!(normalize_easy_command("hello world"), "hello world");
}
