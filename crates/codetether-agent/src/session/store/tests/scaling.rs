//! Work accounting, not wall-clock assertions: appends serialize only their delta.
use super::super::{batch, connection, load, save};
use super::message;
use crate::session::Session;
#[tokio::test]
async fn append_and_noop_cost_do_not_grow_with_history() {
    let mut bytes = Vec::new();
    for size in [100, 10_000] {
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("scale.json");
        let mut session = Session::new().await.unwrap();
        session
            .messages
            .extend((0..size).map(|_| message("historic")));
        save(&session, &path).await.unwrap();
        session.messages.push(message("delta"));
        let checkpoint = session.storage.0.lock().unwrap().clone();
        let pending = batch::prepare(&session, &checkpoint).unwrap();
        assert_eq!(pending.messages.start, size);
        assert_eq!(pending.messages.rows.len(), 1);
        bytes.push(pending.messages.rows[0].len());
        eprintln!(
            "history={size} appended_records=1 encoded_bytes={}",
            pending.messages.rows[0].len()
        );
        save(&session, &path).await.unwrap();
        let checkpoint = session.storage.0.lock().unwrap().clone();
        let pending = batch::prepare(&session, &checkpoint).unwrap();
        assert!(pending.unchanged(&checkpoint));
        assert!(pending.messages.rows.is_empty());
        let tail = load(&path, 3).await.unwrap();
        assert_eq!(tail.session.messages.len(), 3);
        assert_eq!(tail.session.message_offset(), size - 2);
        let db = connection::open(&path).unwrap();
        let plan: String = db.query_row("EXPLAIN QUERY PLAN SELECT body FROM records WHERE session_id='x' AND kind=0 AND seq>=10 AND seq<13 ORDER BY seq", [], |r| r.get(3)).unwrap();
        assert!(plan.contains("SEARCH records USING PRIMARY KEY"), "{plan}");
        assert!(!plan.contains("SCAN records"));
    }
    assert_eq!(bytes[0], bytes[1]);
}
