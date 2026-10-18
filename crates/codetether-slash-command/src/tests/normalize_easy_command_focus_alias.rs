//! Regression coverage retained from native command parsing.
use crate::normalize_easy_command;

#[test]
fn normalize_easy_command_focus_alias() {
    assert_eq!(normalize_easy_command("/focus coder"), "/agent coder");
    assert_eq!(normalize_easy_command("/focus @coder"), "/agent coder");
    assert_eq!(normalize_easy_command("/focus"), "/agent");
}
