//! Regression coverage retained from native command parsing.
use crate::normalize_easy_command;

#[test]
fn normalize_easy_command_talk_alias() {
    assert_eq!(
        normalize_easy_command("/talk coder hello"),
        "/agent coder hello"
    );
    assert_eq!(
        normalize_easy_command("/say coder hello"),
        "/agent coder hello"
    );
    assert_eq!(normalize_easy_command("/talk"), "/agent");
}
