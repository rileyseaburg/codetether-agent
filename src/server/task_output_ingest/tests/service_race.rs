//! Concurrent real services preserve process-local progress/completion ordering.

use super::{
    completion, event_assertions,
    fixtures::{payload, setup},
};
use crate::server::{task_queue::OutputError, worker_modules::task_release_service};
use std::sync::Arc;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn concurrent_progress_and_release_never_publish_progress_after_completion() {
    for status in ["completed", "failed"] {
        for _ in 0..32 {
            let (tasks, bus, mut reader) = setup("processing").await;
            let barrier = Arc::new(tokio::sync::Barrier::new(2));
            let progress_tasks = tasks.clone();
            let progress_bus = bus.clone();
            let progress_barrier = barrier.clone();
            let progress = tokio::spawn(async move {
                progress_barrier.wait().await;
                super::super::service::ingest(&progress_tasks, &progress_bus, "task-1", &payload())
                    .await
            });
            barrier.wait().await;
            task_release_service::release(&tasks, &bus, &completion::request(status))
                .await
                .unwrap();
            let accepted = match progress.await.unwrap() {
                Ok(()) => true,
                Err(error) => {
                    assert_eq!(error, OutputError::NotActive);
                    false
                }
            };
            let task = tasks.get("task-1").await.unwrap();
            assert_eq!(task.status, status);
            assert_eq!(
                task.completion.session_id.as_deref(),
                Some("original-session")
            );
            event_assertions::ordered(&mut reader, status, accepted);
        }
    }
}
