//! Regression coverage retained from native command parsing.
use crate::command_with_optional_args;

#[test]
fn command_with_optional_args_avoids_prefix_collisions() {
    assert_eq!(command_with_optional_args("/filed", "/file"), None);
}
