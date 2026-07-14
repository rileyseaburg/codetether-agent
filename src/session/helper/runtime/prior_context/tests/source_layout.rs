use super::user;
use crate::session::helper::runtime::prior_context_allowed;

#[test]
fn later_line_repository_restriction_is_recognized() {
    let messages = [
        user("continue\nuse the repository docs as the source of truth"),
        user("fix it"),
    ];
    assert!(!prior_context_allowed(&messages));
}

#[test]
fn bulleted_repository_restriction_is_an_instruction() {
    let messages = [user("instructions:\n- use the repository docs"), user("go")];
    assert!(!prior_context_allowed(&messages));
}

#[test]
fn instruction_after_example_block_is_recognized() {
    let messages = [
        user("examples:\nenable session recall\n\nuse the repository docs"),
        user("continue"),
    ];
    assert!(!prior_context_allowed(&messages));
}
