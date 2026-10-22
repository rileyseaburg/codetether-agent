mod constraint_support;
mod summary_cursor_support;
use codetether_session_store::{consumer::start_at_tail, save};
use constraint_support::Session;
use summary_cursor_support::next;
#[tokio::test]
async fn new_view_starts_near_tail_and_restarts_do_not_skip_pending_work() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("session.json");
    let mut session = Session::default();
    for _ in 0..250 {
        session.append("source remains available");
    }
    save(&session, &path).await.unwrap();
    start_at_tail(&path, "constraints-session", "rlm:v2", 128, 16).unwrap();
    assert_eq!(next(&path, "rlm:v2").unwrap().ticket.from, 112);
    start_at_tail(&path, "constraints-session", "rlm:v2", 16, 16).unwrap();
    assert_eq!(next(&path, "rlm:v2").unwrap().ticket.from, 112);
    assert_eq!(next(&path, "archive").unwrap().ticket.from, 0);
    assert!(start_at_tail(&path, "constraints-session", "invalid", 0, 16).is_err());
}
