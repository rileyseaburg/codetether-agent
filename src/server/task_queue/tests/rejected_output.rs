//! Rejected progress neither mutates terminal receipts nor notifies.

use super::task;
use crate::server::task_queue::{KnativeTaskQueue, OutputError, TaskCompletion};

#[tokio::test]
async fn rejected_output_preserves_terminal_receipts() {
    for status in ["completed", "failed", "cancelled", "canceled", "unknown"] {
        let queue = KnativeTaskQueue::new();
        let mut original = task(status);
        original.completion = TaskCompletion {
            result: Some("receipt".into()),
            session_id: Some("session-1".into()),
            ..Default::default()
        };
        queue.push(original.clone()).await;
        let error = queue
            .record_output("task-1", |_| panic!("rejected output notified"))
            .await
            .unwrap_err();
        assert_eq!(error, OutputError::NotActive);
        assert_eq!(
            serde_json::to_value(queue.get("task-1").await.unwrap()).unwrap(),
            serde_json::to_value(original).unwrap()
        );
    }
}

#[tokio::test]
async fn missing_output_preserves_other_tasks() {
    let queue = KnativeTaskQueue::new();
    let original = task("processing");
    queue.push(original.clone()).await;
    let error = queue
        .record_output("absent", |_| panic!("missing output notified"))
        .await
        .unwrap_err();
    assert_eq!(error, OutputError::NotFound);
    assert_eq!(
        serde_json::to_value(queue.list().await).unwrap(),
        serde_json::to_value(vec![original]).unwrap()
    );
}
