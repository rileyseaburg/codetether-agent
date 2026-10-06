//! Reconnect snapshots must be reconciled with current queue state.

use super::super::WorkerStream;
use super::{body, fixtures::task};
use crate::server::KnativeTaskQueue;
use futures::StreamExt;
use tokio::sync::broadcast;

#[tokio::test]
async fn snapshot_omits_tasks_claimed_after_subscription() {
    let queue = KnativeTaskQueue::new();
    let mut claimed = task();
    claimed.task_id = "claimed".into();
    let mut available = task();
    available.task_id = "available".into();
    queue.push(claimed).await;
    queue.push(available).await;
    let snapshot = queue.list().await;
    assert!(queue.update_status("claimed", "working").await);
    let (sender, receiver) = broadcast::channel(2);
    drop(sender);
    let stream = WorkerStream::new(snapshot, receiver, queue);
    let payload = body::first(stream).await;
    assert!(payload.contains("available"));
    assert!(!payload.contains("claimed"));
}

#[tokio::test]
async fn snapshot_does_not_resurrect_removed_tasks() {
    let queue = KnativeTaskQueue::new();
    queue.push(task()).await;
    let snapshot = queue.list().await;
    assert!(queue.pop().await.is_some());
    let (sender, receiver) = broadcast::channel(2);
    drop(sender);
    let stream = WorkerStream::new(snapshot, receiver, queue);
    assert!(Box::pin(stream).next().await.is_none());
}
