use super::super::super::persistence;
use super::test_support as support;
use crate::provider::{ContentPart, Role};
use crate::session::{PageKind, Session};

#[tokio::test]
async fn enabled_marker_is_developer_visible_and_durable() {
    assert_marker(true, true).await;
}

#[tokio::test]
async fn disabled_marker_leaves_history_unchanged() {
    assert_marker(false, false).await;
}

async fn assert_marker(enabled: bool, expected: bool) {
    let (_temp, _guard) = persistence::test_support::isolate();
    let owner = format!("owner-{}", uuid::Uuid::new_v4());
    let entry = support::child(&owner).await;
    support::start(entry.id());
    assert!(
        super::handle(entry.id(), Some(&owner), enabled)
            .await
            .unwrap()
            .success
    );
    let session = Session::load(entry.id()).await.unwrap();
    assert_eq!(session.messages.len(), usize::from(expected));
    if expected {
        assert_eq!(session.messages[0].role, Role::Developer);
        assert_eq!(session.pages[0], PageKind::Bootstrap);
        assert!(
            matches!(&session.messages[0].content[0], ContentPart::Text { text }
            if text == super::marker::TEXT)
        );
    }
    support::cleanup(entry.id());
}
