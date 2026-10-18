//! Payload-bearing events retain earlier outcomes rather than only save markers.
use super::super::{connection, save};
use super::message;
use crate::session::Session;
#[tokio::test]
async fn edits_retain_old_payload_and_append_cost_is_constant() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("events.json");
    let mut session = Session::new().await.unwrap();
    session
        .messages
        .extend((0..100).map(|_| message("original")));
    save(&session, &path).await.unwrap();
    session.messages.push(message("added"));
    save(&session, &path).await.unwrap();
    let db = connection::open(&path).unwrap();
    let added: usize = db
        .query_row("SELECT count(*) FROM events WHERE revision=2", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert_eq!(added, 4); // header, two truncation boundaries, one new record
    session.messages[0] = message("edited");
    save(&session, &path).await.unwrap();
    let old: String = db
        .query_row(
            "SELECT body FROM events WHERE revision=1 AND kind=0 AND seq=0 AND op='put'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert!(old.contains("original"));
    let current: String = db
        .query_row("SELECT body FROM records WHERE kind=0 AND seq=0", [], |r| {
            r.get(0)
        })
        .unwrap();
    assert!(current.contains("edited"));
    let before: usize = db
        .query_row("SELECT count(*) FROM events", [], |r| r.get(0))
        .unwrap();
    assert!(!save(&session, &path).await.unwrap());
    let after: usize = db
        .query_row("SELECT count(*) FROM events", [], |r| r.get(0))
        .unwrap();
    assert_eq!(before, after);
}
