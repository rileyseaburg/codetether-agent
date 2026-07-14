use super::user;
use crate::session::helper::runtime::prior_context_allowed;

#[test]
fn varied_opt_outs_persist_across_followups() {
    for denial in [
        "don't use session recall",
        "never inspect previous-session data",
        "use the repository docs, not session recall",
        "do this without session recall",
        "i didnt ask you to look at scope or session its in the docs",
        "do not inspect scope; use the repository docs",
    ] {
        let messages = vec![user(denial), user("fix the next issue")];
        assert!(!prior_context_allowed(&messages), "{denial}");
    }
}

#[test]
fn one_turn_allow_does_not_erase_persistent_denial() {
    let mut messages = vec![user("do not use session recall")];
    messages.push(user("check the previous session this time"));
    assert!(prior_context_allowed(&messages));
    messages.push(user("continue"));
    assert!(!prior_context_allowed(&messages));
}

#[test]
fn explicit_reenable_persists() {
    let messages = vec![
        user("do not use session recall"),
        user("you can use session recall again"),
        user("go"),
    ];
    assert!(prior_context_allowed(&messages));
}

#[test]
fn mentions_and_source_paths_are_not_directives() {
    for text in [
        "why does session recall keep happening",
        "fix session_recall",
        "look in src/session",
        "run session tests",
    ] {
        assert!(prior_context_allowed(&[user(text)]), "{text}");
    }
}
