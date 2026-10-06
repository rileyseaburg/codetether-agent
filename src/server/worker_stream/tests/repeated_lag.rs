//! Each lag recovery must consult current state, not retain previously queued work.

use super::{super::WorkerStream, body, fixtures};
use crate::server::KnativeTaskQueue;
use futures::StreamExt;
use std::time::Duration;
use tokio::sync::broadcast;

#[tokio::test]
async fn repeated_lag_does_not_revive_claimed_work() {
    let queue = KnativeTaskQueue::default();
    let task = fixtures::task();
    queue.push(task.clone()).await;
    let (sender, receiver) = broadcast::channel(1);
    let mut stream = Box::pin(WorkerStream::new(Vec::new(), receiver, queue.clone()));
    for expected in ["event: lag", "event: task"] {
        if expected == "event: lag" {
            sender.send(fixtures::notification("missing-1")).unwrap();
            sender.send(fixtures::notification("missing-2")).unwrap();
        }
        let event = tokio::time::timeout(Duration::from_secs(1), stream.next())
            .await
            .unwrap()
            .unwrap();
        let wire = body::first(futures::stream::once(async move { event })).await;
        assert!(wire.starts_with(expected));
    }
    queue.claim(&task.task_id).await.unwrap();
    sender.send(fixtures::notification("missing-3")).unwrap();
    sender.send(fixtures::notification("missing-4")).unwrap();
    drop(sender);
    let wire = body::events(stream, 8).await;
    assert!(wire.starts_with("event: lag\ndata: {\"skipped\":2}"));
    assert!(!wire.contains("event: task"));
    assert_eq!(queue.get(&task.task_id).await.unwrap().status, "processing");
}
