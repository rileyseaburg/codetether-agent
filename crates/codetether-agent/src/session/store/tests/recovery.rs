//! Transactions protect the previous revision and unknown tool outcomes.
use super::super::{batch, connection, load, save, transaction};
use super::message;
use crate::session::Session;
#[tokio::test]
async fn failed_transaction_rolls_back_and_retry_does_not_duplicate() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("atomic.json");
    let mut session = Session::new().await.unwrap();
    session.messages.push(message("safe"));
    save(&session, &path).await.unwrap();
    session.messages.push(message("next"));
    let checkpoint = session.storage.0.lock().unwrap().clone();
    let mut pending = batch::prepare(&session, &checkpoint).unwrap();
    pending.messages.rows[0] = "not a message".into();
    assert!(transaction::commit(&path, &pending).is_err());
    assert_eq!(load(&path, 10).await.unwrap().session.messages.len(), 1);
    let pending = batch::prepare(&session, &checkpoint).unwrap();
    assert_eq!(transaction::commit(&path, &pending).unwrap(), 2);
    assert_eq!(transaction::commit(&path, &pending).unwrap(), 2);
    assert_eq!(load(&path, 10).await.unwrap().session.messages.len(), 2);
}
#[tokio::test]
async fn missing_tool_results_remain_unknown_and_large_payloads_are_blobs() {
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("tools.json");
    let mut session = Session::new().await.unwrap();
    session.messages.push(
        serde_json::from_value(serde_json::json!({"role":"assistant","content":[
            {"type":"tool_call","id":"call-1","name":"exec_command","arguments":"{}"}
        ]}))
        .unwrap(),
    );
    session.messages.push(message(&"x".repeat(100_000)));
    save(&session, &path).await.unwrap();
    let db = connection::open(&path).unwrap();
    let state: String = db
        .query_row("SELECT state FROM tool_calls", [], |r| r.get(0))
        .unwrap();
    assert_eq!(state, "unknown");
    let count: usize = db
        .query_row("SELECT count(*) FROM blobs", [], |r| r.get(0))
        .unwrap();
    assert_eq!(count, 1);
    let full = load(&path, 10).await.unwrap().session;
    assert_eq!(
        serde_json::to_value(&full.messages).unwrap(),
        serde_json::to_value(&session.messages).unwrap()
    );
}
