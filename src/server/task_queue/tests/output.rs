//! Accepted progress preserves task fields and notifies the working snapshot.

use super::task;
use crate::server::task_queue::KnativeTaskQueue;
use std::cell::Cell;

#[tokio::test]
async fn output_accepts_legacy_active_states_without_losing_payload() {
    for status in ["pending", "queued", "processing", "working"] {
        let queue = KnativeTaskQueue::new();
        let mut expected = task(status);
        expected.model = Some("test-model".into());
        expected.metadata = Some(serde_json::json!({"source": "fixture"}));
        queue.push(expected.clone()).await;
        let notified = Cell::new(false);
        let updated = queue
            .record_output("task-1", |snapshot| {
                assert_eq!(snapshot.status, "working");
                notified.set(true);
            })
            .await
            .unwrap();
        expected.status = "working".into();
        assert!(notified.get());
        assert_eq!(
            serde_json::to_value(updated).unwrap(),
            serde_json::to_value(expected).unwrap()
        );
    }
}
