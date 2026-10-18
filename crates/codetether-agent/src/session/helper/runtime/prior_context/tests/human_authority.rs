use super::user;
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use crate::session::helper::runtime::prior_context::directive::persistent_update;
use crate::session::helper::runtime::prior_context_allowed_for_session;

fn assistant(text: &str) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}

#[tokio::test]
async fn synthetic_user_text_cannot_reenable_persistent_denial() {
    let mut session = Session::new().await.expect("session");
    session.add_human_message(user("never use session recall"));
    session.add_message(user("you can use session recall again"));
    assert!(!prior_context_allowed_for_session(&session));
    assert_eq!(session.metadata.prior_context_allowed, Some(false));
}

#[tokio::test]
async fn synthetic_user_text_does_not_expire_one_turn_denial() {
    let mut session = Session::new().await.expect("session");
    session.add_human_message(user("do not use session recall this turn"));
    session.add_message(user("you can use session recall again"));
    assert!(!prior_context_allowed_for_session(&session));
    session.add_human_message(user("continue with the code"));
    assert!(prior_context_allowed_for_session(&session));
}

#[tokio::test]
async fn assistant_prose_cannot_initialize_persistent_policy() {
    let mut session = Session::new().await.expect("session");
    let assistant = assistant(
        "I’m inspecting the diff for scope creep, then finishing without committing unrelated changes.",
    );
    assert_eq!(persistent_update(&assistant), Some(false));
    session.add_message(assistant);
    session.add_human_message(user("status?"));
    assert_eq!(session.metadata.prior_context_allowed, Some(true));
    assert!(prior_context_allowed_for_session(&session));
}
