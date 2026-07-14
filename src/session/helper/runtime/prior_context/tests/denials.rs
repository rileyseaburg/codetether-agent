use super::user;
use crate::session::helper::runtime::prior_context_allowed;

#[test]
fn natural_denials_persist_without_access_verbs() {
    for denial in [
        "avoid prior context",
        "no session history",
        "stop touching previous sessions",
        "do not access session history",
        "you must not use session recall",
        "you cannot use session recall",
        "turn off session recall",
        "disable session recall",
        "block session recall",
        "revoke session recall access",
        "opt out of session recall",
        "no scope access",
        "no history access",
        "no prior context access",
        "do not use `session_recall`",
        "never inspect the \"session history\"",
    ] {
        let messages = [user(denial), user("keep fixing the code")];
        assert!(!prior_context_allowed(&messages), "{denial}");
    }
}

#[test]
fn unmatched_quote_keeps_an_explicit_denial_target() {
    let messages = [user("do not use \"session recall"), user("fix it")];
    assert!(!prior_context_allowed(&messages));
}
