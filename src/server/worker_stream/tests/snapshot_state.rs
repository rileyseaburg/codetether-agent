//! Reconnect delivery rechecks authoritative state, never stale snapshot data.

use super::{super::WorkerStream, body, fixtures};
use crate::server::KnativeTaskQueue;
use futures::StreamExt;
use tokio::sync::broadcast;

#[tokio::test]
async fn reconnect_skips_terminal_claimed_and_missing_tasks() {
    for status in ["working", "completed", "failed", "cancelled", "missing"] {
        let queue = KnativeTaskQueue::new();
        let snapshot = fixtures::task();
        if status != "missing" {
            let mut current = snapshot.clone();
            current.status = status.into();
            queue.push(current).await;
        }
        let (tx, rx) = broadcast::channel(1);
        drop(tx);
        let stream = WorkerStream::new(vec![snapshot], rx, queue);
        futures::pin_mut!(stream);
        assert!(stream.next().await.is_none(), "stale status: {status}");
    }
}

#[tokio::test]
async fn reconnect_serializes_current_payload_not_snapshot() {
    let queue = KnativeTaskQueue::new();
    let snapshot = fixtures::task();
    let mut current = snapshot.clone();
    current.model = Some("provider/updated-model".into());
    current.metadata = Some(serde_json::json!({"codebase_id": "updated-workspace"}));
    queue.push(current).await;
    let (_tx, rx) = broadcast::channel(1);
    let frame = body::first(WorkerStream::new(vec![snapshot], rx, queue)).await;
    let json = frame
        .lines()
        .find_map(|line| line.strip_prefix("data: "))
        .unwrap();
    let payload: serde_json::Value = serde_json::from_str(json).unwrap();
    assert_eq!(payload["model"], "provider/updated-model");
    assert_eq!(payload["metadata"]["codebase_id"], "updated-workspace");
    assert_eq!(payload["status"], "pending");
}
