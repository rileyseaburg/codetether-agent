//! A subscribed idle stream wakes for work enqueued after its initial snapshot.

use super::{super::WorkerStream, body, fixtures};
use crate::server::KnativeTaskQueue;
use futures::StreamExt;
use std::task::Poll;
use tokio::sync::broadcast;

#[tokio::test]
async fn idle_stream_delivers_new_authoritative_payload() {
    let queue = KnativeTaskQueue::default();
    let (sender, receiver) = broadcast::channel(8);
    let mut stream = Box::pin(WorkerStream::new(Vec::new(), receiver, queue.clone()));
    assert!(matches!(futures::poll!(stream.next()), Poll::Pending));

    let mut task = fixtures::task();
    task.model = Some("new-provider/new-model".into());
    task.metadata = Some(serde_json::json!({"codebase_id": "new-workspace"}));
    queue.push(task.clone()).await;
    sender.send(fixtures::notification(&task.task_id)).unwrap();

    let wire = body::first(stream).await;
    let data = wire
        .lines()
        .find_map(|line| line.strip_prefix("data: "))
        .unwrap();
    let payload: serde_json::Value = serde_json::from_str(data).unwrap();
    assert!(wire.starts_with("event: task\n"));
    assert_eq!(payload["task_id"], task.task_id);
    assert_eq!(payload["model"], "new-provider/new-model");
    assert_eq!(payload["metadata"]["codebase_id"], "new-workspace");
    assert_eq!(queue.get(&task.task_id).await.unwrap().status, "pending");
}
