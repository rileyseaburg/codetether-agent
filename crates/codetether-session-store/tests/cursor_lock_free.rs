//! Polling an existing cursor must not queue behind the cross-process writer lock.
mod constraint_support;
mod summary_cursor_support;
use codetether_session_store::{consumer::start_at_tail, save};
use constraint_support::Session;
use fs2::FileExt;
use std::time::{Duration, Instant};
use summary_cursor_support::next;

#[tokio::test]
async fn existing_cursor_polls_while_another_process_holds_the_writer_lock() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.json");
    let mut session = Session::default();
    for _ in 0..48 {
        session.append("source remains available");
    }
    save(&session, &path).await.unwrap();
    start_at_tail(&path, "constraints-session", "rlm:v2", 32, 16).unwrap();
    assert_eq!(next(&path, "rlm:v2").unwrap().ticket.from, 16);
    let lock = std::fs::File::open(root.path().join("sessions.sqlite3.writer.lock")).unwrap();
    FileExt::lock_exclusive(&lock).unwrap();
    let started = Instant::now();
    start_at_tail(&path, "constraints-session", "rlm:v2", 32, 16).unwrap();
    assert_eq!(next(&path, "rlm:v2").unwrap().ticket.from, 16);
    assert!(started.elapsed() < Duration::from_secs(2));
}
