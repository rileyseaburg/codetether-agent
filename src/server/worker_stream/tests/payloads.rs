//! Snapshot and live notifications must use the same worker task schema.

use super::super::WorkerStream;
use super::{body, fixtures};
use crate::server::KnativeTaskQueue;
use tokio::sync::broadcast;

#[tokio::test]
async fn snapshot_and_live_payloads_preserve_execution_configuration() {
    let task = fixtures::task();
    let queue = KnativeTaskQueue::new();
    queue.push(task.clone()).await;
    let (tx, rx) = broadcast::channel(8);
    let snapshot = body::first(WorkerStream::new(vec![task.clone()], rx, queue.clone())).await;
    let live_rx = tx.subscribe();
    tx.send(fixtures::notification(&task.task_id)).unwrap();
    let live = body::first(WorkerStream::new(Vec::new(), live_rx, queue)).await;
    assert_eq!(snapshot, live);
    assert!(live.starts_with("event: task\n"));
    let json = live
        .lines()
        .find_map(|line| line.strip_prefix("data: "))
        .unwrap();
    let payload: serde_json::Value = serde_json::from_str(json).unwrap();
    assert_eq!(payload["task_id"], task.task_id);
    assert_eq!(payload["model"], "provider/model");
    assert_eq!(payload["metadata"]["codebase_id"], "workspace-1");
    assert!(payload.get("message").is_none());
    assert!(payload.get("topic").is_none());
}
