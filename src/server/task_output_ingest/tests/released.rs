//! Real service-level terminal receipts survive rejected late progress.

use super::{
    completion, event_assertions,
    fixtures::{payload, setup},
};
use crate::server::{task_queue::OutputError, worker_modules::task_release_service};

#[tokio::test]
async fn released_tasks_reject_late_progress_without_receipt_or_event_changes() {
    for status in ["completed", "failed"] {
        let (tasks, bus, mut reader) = setup("processing").await;
        let request = completion::request(status);
        task_release_service::release(&tasks, &bus, &request)
            .await
            .unwrap();
        let before = serde_json::to_value(tasks.get("task-1").await.unwrap()).unwrap();
        assert_eq!(before["status"], status);
        assert_eq!(before["session_id"], "original-session");
        assert_eq!(before["result"], "final result");
        assert_eq!(before["diagnostics"]["phase"], "done");
        assert_eq!(
            super::super::service::ingest(&tasks, &bus, "task-1", &payload()).await,
            Err(OutputError::NotActive),
        );
        let after = serde_json::to_value(tasks.get("task-1").await.unwrap()).unwrap();
        assert_eq!(before, after);
        event_assertions::ordered(&mut reader, status, false);
    }
}
