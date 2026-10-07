//! Deterministic service ordering and duplicate terminal notification rejection.

use super::{
    completion, event_assertions,
    fixtures::{payload, setup},
};
use crate::server::{
    task_queue::{OutputError, ReleaseError},
    worker_modules::task_release_service,
};

#[tokio::test]
async fn accepted_progress_precedes_terminal_receipt_and_retries_emit_nothing() {
    for status in ["completed", "failed"] {
        let (tasks, bus, mut reader) = setup("processing").await;
        super::super::service::ingest(&tasks, &bus, "task-1", &payload())
            .await
            .unwrap();
        let request = completion::request(status);
        task_release_service::release(&tasks, &bus, &request)
            .await
            .unwrap();
        let before = serde_json::to_value(tasks.get("task-1").await.unwrap()).unwrap();
        assert_eq!(before["status"], status);
        assert_eq!(before["result"], "final result");
        event_assertions::ordered(&mut reader, status, true);
        let mut retry = completion::request(status);
        retry.result = Some("must not replace the winning receipt".into());
        retry.session_id = Some("replacement-session".into());
        assert_eq!(
            task_release_service::release(&tasks, &bus, &retry).await,
            Err(ReleaseError::NotActive),
        );
        assert_eq!(
            super::super::service::ingest(&tasks, &bus, "task-1", &payload()).await,
            Err(OutputError::NotActive),
        );
        assert_eq!(
            serde_json::to_value(tasks.get("task-1").await.unwrap()).unwrap(),
            before,
        );
        assert!(
            reader.try_recv().is_none(),
            "rejected retries must not notify"
        );
    }
}
