//! Replay snapshots cannot resurrect tasks that stop being deliverable.

use super::{super::WorkerStream, fixtures};
use crate::server::KnativeTaskQueue;
use futures::StreamExt;
use tokio::sync::broadcast;

#[tokio::test]
async fn lag_replay_rechecks_terminal_and_removed_tasks() {
    for status in ["completed", "failed", "cancelled", "working", "removed"] {
        let queue = KnativeTaskQueue::default();
        let task = fixtures::task();
        queue.push(task.clone()).await;
        let (sender, receiver) = broadcast::channel(1);
        sender.send(fixtures::notification("lost")).unwrap();
        sender.send(fixtures::notification("unresolved")).unwrap();
        drop(sender);
        let stream = WorkerStream::new(Vec::new(), receiver, queue.clone());
        futures::pin_mut!(stream);
        assert!(stream.next().await.is_some()); // lag captures the replay snapshot
        if status == "removed" {
            queue.pop().await.unwrap();
        } else {
            assert!(queue.update_status(&task.task_id, status).await);
        }
        assert!(
            tokio::time::timeout(std::time::Duration::from_secs(1), stream.next())
                .await
                .unwrap()
                .is_none(),
            "stale replay snapshot delivered a {status} task"
        );
    }
}
