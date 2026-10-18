//! Regression coverage retained from native command parsing.
use crate::normalize_easy_command;

#[test]
fn normalize_easy_command_home_alias() {
    assert_eq!(normalize_easy_command("/home"), "/chat");
    assert_eq!(normalize_easy_command("/main"), "/chat");
}
