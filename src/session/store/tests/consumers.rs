//! Consumers retry unacknowledged deltas; edits fence in-flight acknowledgements.
use super::super::{cursor, cursor_read, projection, save};
use super::message;
use crate::session::Session;
#[tokio::test]
async fn independent_cursors_and_edit_invalidation() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("cursors.json");
    let mut session = Session::new().await.unwrap();
    session.messages.extend((0..20).map(|_| message("old")));
    save(&session, &path).await.unwrap();
    let first = cursor_read::next(&path, &session.id, "recall", 8)
        .unwrap()
        .unwrap();
    let retry = cursor_read::next(&path, &session.id, "recall", 8)
        .unwrap()
        .unwrap();
    assert_eq!(first.ticket.from, retry.ticket.from);
    assert!(cursor::acknowledge(first.ticket).await.unwrap());
    let next = cursor_read::next(&path, &session.id, "recall", 8)
        .unwrap()
        .unwrap();
    assert_eq!(next.ticket.from, 8);
    let archive = cursor_read::next(&path, &session.id, "archive", 8)
        .unwrap()
        .unwrap();
    assert_eq!(archive.ticket.from, 0);
    session.messages[3] = message("edited");
    save(&session, &path).await.unwrap();
    assert!(!cursor::acknowledge(archive.ticket).await.unwrap());
    assert!(
        !projection::commit(next.ticket, serde_json::json!({"documents":[]}))
            .await
            .unwrap()
    );
    let corrected = cursor_read::next(&path, &session.id, "recall", 8)
        .unwrap()
        .unwrap();
    assert_eq!(corrected.ticket.from, 0);
    assert!(save(&corrected.session, &path).await.is_err());
}
