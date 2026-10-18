//! Empty legacy payloads update lifecycle state without fabricating output.

use super::super::service::ingest;
use super::fixtures::setup;

#[tokio::test]
async fn empty_legacy_output_updates_state_without_inventing_event() {
    let (tasks, bus, mut reader) = setup("pending").await;
    let empty = serde_json::from_str("{}").unwrap();
    ingest(&tasks, &bus, "task-1", &empty).await.unwrap();
    assert_eq!(tasks.get("task-1").await.unwrap().status, "working");
    assert!(reader.try_recv().is_none());
}
