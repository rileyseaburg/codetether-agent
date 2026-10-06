//! Rejected releases cannot replace any part of the accepted receipt.

use super::task;
use crate::server::task_queue::{KnativeTaskQueue, ReleaseError, TaskCompletion};

pub(super) fn receipt() -> TaskCompletion {
    TaskCompletion {
        result: Some("accepted result".into()),
        error: Some("nonfatal detail".into()),
        session_id: Some("original-session".into()),
        diagnostics: Some(serde_json::json!({"attempt": 1, "nested": [true]})),
    }
}

#[tokio::test]
async fn rejected_releases_preserve_the_entire_accepted_task() {
    for status in ["completed", "failed"] {
        let queue = KnativeTaskQueue::new();
        queue.push(task("working")).await;
        let accepted = queue
            .release_with_completion("task-1", status, receipt())
            .await
            .unwrap();
        let snapshot = serde_json::to_value(accepted).unwrap();
        for (next, expected) in [
            ("failed", ReleaseError::NotActive),
            ("completed", ReleaseError::NotActive),
            ("working", ReleaseError::InvalidStatus),
        ] {
            assert_eq!(
                queue
                    .release_with_completion("task-1", next, TaskCompletion::default())
                    .await
                    .unwrap_err(),
                expected
            );
            assert_eq!(
                serde_json::to_value(queue.get("task-1").await.unwrap()).unwrap(),
                snapshot
            );
        }
    }
}
