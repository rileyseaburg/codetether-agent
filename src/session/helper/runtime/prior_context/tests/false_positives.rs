use super::user;
use crate::session::helper::runtime::prior_context_allowed;

#[test]
fn ordinary_code_commands_do_not_unlock_prior_context() {
    for command in [
        "check memory usage",
        "open session settings",
        "use scope guards",
        "check session recall implementation",
        "look at session recall code",
        "read previous session migration tests",
        "check session recall implementation again",
        "check memory",
        "check session recall",
        "check session recall again",
    ] {
        let messages = [user("never use session recall"), user(command)];
        assert!(!prior_context_allowed(&messages), "{command}");
    }
}

#[test]
fn quoted_permission_text_does_not_override_denial() {
    let messages = [
        user("never use session recall"),
        user("add a test for \"you can use session recall again\""),
    ];
    assert!(!prior_context_allowed(&messages));
}

#[test]
fn polite_mentions_and_failure_questions_are_not_directives() {
    for text in [
        "please explain why session recall keeps happening",
        "why is session recall not working",
    ] {
        assert!(prior_context_allowed(&[user(text)]), "{text}");
    }
}
