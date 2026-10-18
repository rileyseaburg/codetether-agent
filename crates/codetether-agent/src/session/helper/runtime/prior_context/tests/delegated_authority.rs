use super::user;
use crate::session::Session;
use crate::session::helper::runtime::prior_context_allowed_for_session;

#[tokio::test]
async fn delegated_messages_can_tighten_but_not_elevate_access() {
    let mut session = Session::new().await.expect("session");
    session.add_delegated_message(user("never use session recall"));
    assert!(!prior_context_allowed_for_session(&session));

    session.add_delegated_message(user("enable session recall"));
    assert!(!prior_context_allowed_for_session(&session));
}
