//! Cancelled saves leave checkpoints dirty and retry the same durable identity.
mod constraint_support;
#[path = "writer_support/hold.rs"]
mod hold;
use codetether_session_store::{connection, coordinator, save, transaction};
use std::{
    future::Future,
    task::{Context, Waker},
};

#[tokio::test]
async fn cancelled_save_can_acknowledge_its_commit_without_duplicates() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.json");
    drop(connection::open(&path).unwrap());
    let mut session = constraint_support::Session::default();
    session.append("original message");
    let hold = hold::hold(&path);
    let mut pending = Box::pin(save(&session, &path));
    assert!(
        pending
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    assert_eq!(session.state.0.lock().unwrap().revision, 0);
    drop(pending);
    drop(hold);
    coordinator::write_async(&path, |_| Ok(())).await.unwrap();
    assert_eq!(
        transaction::revision(&path, "constraints-session").unwrap(),
        1
    );
    assert!(save(&session, &path).await.unwrap());
    assert_eq!(session.state.0.lock().unwrap().revision, 1);
    let db = connection::open(&path).unwrap();
    let count: i64 = db
        .query_row("SELECT COUNT(*) FROM records", [], |row| row.get(0))
        .unwrap();
    assert_eq!(count, 1);
}
