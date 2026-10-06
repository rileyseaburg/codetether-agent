//! Lag replay reads current execution configuration rather than stale snapshots.

use super::{super::WorkerStream, body, fixtures};
use crate::server::KnativeTaskQueue;
use futures::StreamExt;
use tokio::sync::broadcast;

#[tokio::test]
async fn lag_replay_refreshes_configuration_after_snapshot() {
    let queue = KnativeTaskQueue::default();
    let task = fixtures::task();
    queue.push(task.clone()).await;
    let (sender, receiver) = broadcast::channel(1);
    sender.send(fixtures::notification("lost")).unwrap();
    sender.send(fixtures::notification("unresolved")).unwrap();
    drop(sender);
    let mut stream = Box::pin(WorkerStream::new(Vec::new(), receiver, queue.clone()));
    assert!(stream.next().await.is_some()); // lag captures the old payload

    let mut refreshed = queue.pop().await.unwrap();
    refreshed.model = Some("provider/refreshed-model".into());
    refreshed.metadata = Some(serde_json::json!({"codebase_id": "workspace-2"}));
    refreshed.priority = 11;
    refreshed.status = "queued".into();
    queue.push(refreshed).await;

    let wire = body::first(stream).await;
    let data = wire.strip_prefix("event: task\ndata: ").unwrap();
    let payload: serde_json::Value = serde_json::from_str(data.trim()).unwrap();
    assert_eq!(payload["task_id"], task.task_id);
    assert_eq!(payload["model"], "provider/refreshed-model");
    assert_eq!(payload["metadata"]["codebase_id"], "workspace-2");
    assert_eq!(payload["priority"], 11);
    assert_eq!(payload["status"], "queued");
    assert_eq!(queue.list().await.len(), 1);
    assert_eq!(queue.get(&task.task_id).await.unwrap().status, "queued");
}
