//! Lag is observable and closed channels do not fabricate task events.

use super::{super::WorkerStream, body, fixtures};
use crate::server::KnativeTaskQueue;
use futures::StreamExt;
use tokio::sync::broadcast;

#[tokio::test]
async fn broadcast_lag_reports_skipped_notifications() {
    let (tx, rx) = broadcast::channel(1);
    tx.send(fixtures::notification("one")).unwrap();
    tx.send(fixtures::notification("two")).unwrap();
    let event = body::first(WorkerStream::new(vec![], rx, KnativeTaskQueue::new())).await;
    assert_eq!(event, "event: lag\ndata: {\"skipped\":1}\n\n");
}

#[tokio::test]
async fn closed_channel_drains_snapshot_and_skips_unresolved_live_messages() {
    let queue = KnativeTaskQueue::new();
    let task = fixtures::task();
    queue.push(task.clone()).await;
    let (tx, rx) = broadcast::channel(8);
    tx.send(fixtures::notification("missing")).unwrap();
    tx.send(fixtures::notification(&task.task_id)).unwrap();
    drop(tx);
    let stream = WorkerStream::new(vec![task], rx, queue);
    futures::pin_mut!(stream);
    assert!(stream.next().await.is_some());
    assert!(stream.next().await.is_some());
    assert!(stream.next().await.is_none());
}
