//! Accepted and rejected claims must preserve the task payload.

use super::task;
use crate::server::task_queue::{ClaimError, KnativeTaskQueue};

#[tokio::test]
async fn pending_and_queued_return_processing_snapshots() {
    for status in ["pending", "queued"] {
        let queue = KnativeTaskQueue::new();
        let original = task(status);
        queue.push(original.clone()).await;
        let claimed = queue.claim("task-1").await.expect("claim");
        assert_eq!(claimed.status, "processing");
        assert_eq!(claimed.title, original.title);
        assert_eq!(claimed.description, original.description);
        assert_eq!(claimed.agent_type, original.agent_type);
        assert_eq!(claimed.priority, original.priority);
        assert_eq!(claimed.received_at, original.received_at);
        assert_eq!(queue.get("task-1").await.unwrap().status, "processing");
    }
}

#[tokio::test]
async fn ineligible_claims_do_not_mutate_tasks() {
    for status in ["processing", "completed", "failed", "cancelled", "unknown"] {
        let queue = KnativeTaskQueue::new();
        queue.push(task(status)).await;
        assert_eq!(
            queue.claim("task-1").await.unwrap_err(),
            ClaimError::NotPending
        );
        assert_eq!(queue.get("task-1").await.unwrap().status, status);
    }
}

#[tokio::test]
async fn missing_claim_does_not_mutate_the_queue() {
    let queue = KnativeTaskQueue::new();
    queue.push(task("pending")).await;
    assert_eq!(
        queue.claim("missing").await.unwrap_err(),
        ClaimError::NotFound
    );
    assert_eq!(queue.list().await.len(), 1);
    assert_eq!(queue.get("task-1").await.unwrap().status, "pending");
}
