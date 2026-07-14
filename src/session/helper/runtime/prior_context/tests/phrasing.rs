use super::user;
use crate::session::helper::runtime::prior_context_allowed;

#[test]
fn denial_words_take_precedence_over_permission_words() {
    assert!(!prior_context_allowed(&[user(
        "you can never use session recall again"
    )]));
}

#[test]
fn for_now_denial_survives_arbitrary_followup() {
    let messages = [
        user("do not use session recall for now"),
        user("fix the next thing"),
    ];
    assert!(!prior_context_allowed(&messages));
}

#[test]
fn source_path_does_not_hide_a_denial() {
    let messages = [
        user("do not use session recall; fix src/session/mod.rs"),
        user("continue with that file"),
    ];
    assert!(!prior_context_allowed(&messages));
}

#[test]
fn polite_explicit_permission_is_temporary() {
    let mut messages = vec![user("never inspect previous sessions")];
    messages.push(user("please inspect the previous session this time"));
    assert!(prior_context_allowed(&messages));
    messages.push(user("carry on with the fix"));
    assert!(!prior_context_allowed(&messages));
}
