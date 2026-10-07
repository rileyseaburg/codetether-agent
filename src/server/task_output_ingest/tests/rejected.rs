//! Rejected progress never mutates tasks or publishes process-local events.

use super::super::service::ingest;
use super::fixtures::{payload, setup};
use crate::server::task_queue::OutputError;

#[tokio::test]
async fn late_and_missing_output_never_mutates_or_publishes() {
    for status in ["completed", "failed", "cancelled", "canceled", "unknown"] {
        let (tasks, bus, mut reader) = setup(status).await;
        let before = serde_json::to_value(tasks.get("task-1").await.unwrap()).unwrap();
        assert_eq!(
            ingest(&tasks, &bus, "task-1", &payload()).await,
            Err(OutputError::NotActive)
        );
        assert_eq!(
            ingest(&tasks, &bus, "missing", &payload()).await,
            Err(OutputError::NotFound)
        );
        assert!(tasks.get("missing").await.is_none());
        let after = serde_json::to_value(tasks.get("task-1").await.unwrap()).unwrap();
        assert_eq!(before, after);
        assert!(reader.try_recv().is_none());
    }
}
