//! Regression coverage retained from native command parsing.
use crate::normalize_easy_command;

#[test]
fn normalize_easy_command_add_alias() {
    assert_eq!(normalize_easy_command("/add coder"), "/spawn coder");
    assert_eq!(normalize_easy_command("/add"), "/spawn");
}
