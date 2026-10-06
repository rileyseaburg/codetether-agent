//! At-least-once notifications do not mutate task ownership.

use super::{super::delivery, fixtures};
use crate::server::KnativeTaskQueue;

#[tokio::test]
async fn duplicate_notifications_are_at_least_once_not_claims() {
    let queue = KnativeTaskQueue::new();
    let task = fixtures::task();
    queue.push(task.clone()).await;
    for _ in 0..2 {
        assert!(
            delivery::live(&queue, fixtures::notification(&task.task_id))
                .await
                .is_some()
        );
    }
    assert_eq!(queue.get(&task.task_id).await.unwrap().status, "pending");
}
