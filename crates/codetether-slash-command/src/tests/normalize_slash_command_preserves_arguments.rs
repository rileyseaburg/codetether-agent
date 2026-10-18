//! Regression coverage retained from native command parsing.
use crate::normalize_slash_command;

#[test]
fn normalize_slash_command_preserves_arguments() {
    assert_eq!(normalize_slash_command("/p events"), "/protocol events");
}
