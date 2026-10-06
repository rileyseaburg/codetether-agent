//! Claims racing with lag replay must not cause task redelivery.

use super::{super::WorkerStream, fixtures};
use crate::server::KnativeTaskQueue;
use futures::StreamExt;
use tokio::sync::broadcast;

#[tokio::test]
async fn lag_replay_rechecks_tasks_claimed_after_snapshot() {
    let queue = KnativeTaskQueue::default();
    let task = fixtures::task();
    queue.push(task.clone()).await;
    let (sender, receiver) = broadcast::channel(1);
    sender.send(fixtures::notification("lost")).unwrap();
    sender.send(fixtures::notification("unresolved")).unwrap();
    drop(sender);
    let stream = WorkerStream::new(Vec::new(), receiver, queue.clone());
    futures::pin_mut!(stream);
    assert!(stream.next().await.is_some()); // lag takes the replay snapshot
    assert!(queue.claim(&task.task_id).await.is_ok());
    assert!(
        tokio::time::timeout(std::time::Duration::from_secs(1), stream.next())
            .await
            .unwrap()
            .is_none()
    );
    assert_eq!(queue.get(&task.task_id).await.unwrap().status, "processing");
}
