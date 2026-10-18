//! Regression coverage retained from native command parsing.
use crate::normalize_easy_command;

#[test]
fn normalize_easy_command_list_alias() {
    assert_eq!(normalize_easy_command("/list"), "/agents");
    assert_eq!(normalize_easy_command("/ls"), "/agents");
}
