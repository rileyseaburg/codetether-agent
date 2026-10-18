//! Regression coverage retained from native command parsing.
use crate::normalize_easy_command;

#[test]
fn normalize_easy_command_remove_alias() {
    assert_eq!(normalize_easy_command("/remove coder"), "/kill coder");
    assert_eq!(normalize_easy_command("/rm coder"), "/kill coder");
    assert_eq!(normalize_easy_command("/remove"), "/undo");
}
