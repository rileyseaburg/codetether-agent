//! Prior-context authorization tests for automatic prefetch.

use crate::provider::{ContentPart, Message, Role};

#[tokio::test]
async fn denied_sessions_do_not_prefetch_recall() {
    let mut session = crate::session::Session::new().await.unwrap();
    session.metadata.prior_context_allowed = Some(false);
    session.messages.push(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: "remember the deployment decision".into(),
        }],
    });
    assert!(super::super::prefetch::message(&session).await.is_none());
}

#[test]
fn deletion_tombstone_blocks_and_restore_releases() {
    let id = "recall-delete-race";
    super::super::tombstone::mark(id);
    assert!(super::super::tombstone::contains(id));
    super::super::tombstone::restore(id);
    assert!(!super::super::tombstone::contains(id));
}
