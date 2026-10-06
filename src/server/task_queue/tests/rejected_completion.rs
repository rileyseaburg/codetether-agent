//! Invalid and missing transitions must not store a completion receipt.

use super::{completion::receipt, task};
use crate::server::task_queue::{KnativeTaskQueue, ReleaseError};

#[tokio::test]
async fn invalid_and_missing_releases_do_not_store_a_receipt() {
    let queue = KnativeTaskQueue::new();
    queue.push(task("processing")).await;
    let snapshot = serde_json::to_value(queue.list().await).unwrap();
    assert_eq!(
        queue
            .release_with_completion("task-1", "working", receipt())
            .await
            .unwrap_err(),
        ReleaseError::InvalidStatus
    );
    assert_eq!(
        queue
            .release_with_completion("missing", "failed", receipt())
            .await
            .unwrap_err(),
        ReleaseError::NotFound
    );
    assert_eq!(serde_json::to_value(queue.list().await).unwrap(), snapshot);
}
