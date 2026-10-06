//! Lost notifications replay queued work, without claiming or reviving tasks.

use super::{super::WorkerStream, body, fixtures};
use crate::server::KnativeTaskQueue;
use tokio::sync::broadcast;

#[tokio::test]
async fn lag_replays_configured_pending_tasks_without_claiming() {
    let queue = KnativeTaskQueue::default();
    let pending = fixtures::task();
    queue.push(pending.clone()).await;
    let mut claimed = fixtures::task();
    claimed.task_id = "claimed-task".into();
    claimed.status = "working".into();
    queue.push(claimed).await;
    let (sender, receiver) = broadcast::channel(1);
    sender
        .send(fixtures::notification(&pending.task_id))
        .unwrap();
    sender
        .send(fixtures::notification("unrelated-task"))
        .unwrap();
    drop(sender);

    let wire = body::events(WorkerStream::new(Vec::new(), receiver, queue.clone()), 8).await;
    assert!(wire.starts_with("event: lag\ndata: {\"skipped\":1}"));
    assert_eq!(wire.matches("event: task").count(), 1);
    let data = wire
        .split("event: task\ndata: ")
        .nth(1)
        .expect("replayed task payload")
        .lines()
        .next()
        .unwrap();
    let payload: serde_json::Value = serde_json::from_str(data).unwrap();
    assert_eq!(payload["task_id"], pending.task_id);
    assert_eq!(payload["model"], "provider/model");
    assert_eq!(payload["metadata"]["codebase_id"], "workspace-1");
    assert!(!wire.contains("claimed-task"));
    assert_eq!(queue.get(&pending.task_id).await.unwrap().status, "pending");
    assert_eq!(queue.get("claimed-task").await.unwrap().status, "working");
}
