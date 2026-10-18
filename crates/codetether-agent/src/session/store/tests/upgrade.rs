//! Upgrade keeps development databases readable without deleting prior evidence.
use super::super::{connection, load, save};
use super::message;
use crate::session::Session;
#[tokio::test]
async fn upgrades_v1_without_event_table_and_preserves_records() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("upgrade.json");
    let mut session = Session::new().await.unwrap();
    session.messages.push(message("retained"));
    save(&session, &path).await.unwrap();
    {
        let db = connection::open(&path).unwrap();
        db.execute_batch("DROP TABLE events; DROP TABLE constraints; PRAGMA user_version=1;")
            .unwrap();
    }
    let mut loaded = load(&path, 10).await.unwrap().session;
    assert_eq!(loaded.messages.len(), 1);
    loaded.messages.push(message("new"));
    save(&loaded, &path).await.unwrap();
    let db = connection::open(&path).unwrap();
    let version: i64 = db
        .pragma_query_value(None, "user_version", |r| r.get(0))
        .unwrap();
    assert_eq!(version, 2);
    let events: usize = db
        .query_row("SELECT count(*) FROM events", [], |r| r.get(0))
        .unwrap();
    assert!(events >= 6);
}
