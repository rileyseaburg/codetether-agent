use super::{image_user, user};
use crate::session::helper::runtime::prior_context_allowed;

#[test]
fn one_turn_denial_expires() {
    let mut messages = vec![user("do not use session recall for this turn")];
    assert!(!prior_context_allowed(&messages));
    messages.push(user("continue"));
    assert!(prior_context_allowed(&messages));
}

#[test]
fn direct_recall_request_is_one_turn_permission() {
    let mut messages = vec![
        user("never use session recall"),
        user("inspect the previous session for this request"),
    ];
    assert!(prior_context_allowed(&messages));
    messages.push(user("now fix the docs"));
    assert!(!prior_context_allowed(&messages));
}

#[test]
fn synthetic_image_does_not_expire_one_turn_denial() {
    let messages = [user("do not inspect prior context this turn"), image_user()];
    assert!(!prior_context_allowed(&messages));
}
