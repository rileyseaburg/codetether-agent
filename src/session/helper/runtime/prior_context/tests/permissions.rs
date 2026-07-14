use super::user;
use crate::session::helper::runtime::prior_context_allowed;

#[test]
fn explicit_policy_reenable_phrases_persist() {
    for permission in [
        "allow session recall again",
        "enable session recall",
        "i permit session recall",
        "turn session recall back on",
        "you can use memory again",
        "enable sessions again",
    ] {
        let messages = [
            user("never use session recall"),
            user(permission),
            user("continue with the task"),
        ];
        assert!(prior_context_allowed(&messages), "{permission}");
    }
}

#[test]
fn again_with_this_time_remains_one_turn_permission() {
    let mut messages = vec![user("never use session recall")];
    messages.push(user("inspect the previous session again this time"));
    assert!(prior_context_allowed(&messages));
    messages.push(user("continue without it"));
    assert!(!prior_context_allowed(&messages));
}
