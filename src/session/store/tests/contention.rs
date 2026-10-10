//! Projection commits must wait for writers instead of promoting stale snapshots.
use super::super::{connection, cursor_read, projection, save};
use super::message;
use crate::session::Session;
use std::time::Duration;

#[tokio::test]
async fn projection_waits_for_an_existing_writer() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("contention.json");
    let mut session = Session::new().await.unwrap();
    session.messages.push(message("preserved"));
    save(&session, &path).await.unwrap();
    let ticket = cursor_read::next(&path, &session.id, "recall", 1)
        .unwrap()
        .unwrap()
        .ticket;
    let writer = connection::open(&path).unwrap();
    writer.execute_batch("BEGIN IMMEDIATE").unwrap();
    let mut commit = tokio::spawn(projection::commit(
        ticket,
        serde_json::json!({"documents": []}),
    ));
    let early = tokio::time::timeout(Duration::from_millis(150), &mut commit).await;
    writer.execute_batch("COMMIT").unwrap();
    assert!(early.is_err(), "projection did not wait for writer: {early:?}");
    assert!(commit.await.unwrap().unwrap());
    let position: usize = writer
        .query_row(
            "SELECT seq FROM consumers WHERE session_id=?1 AND name='recall'",
            [&session.id],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(position, 1);
}