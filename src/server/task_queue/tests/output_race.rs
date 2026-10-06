//! Concurrent progress cannot notify after the terminal receipt.

use super::task;
use crate::server::task_queue::{KnativeTaskQueue, OutputError};
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn progress_and_completion_preserve_notification_order() {
    let queue = KnativeTaskQueue::new();
    queue.push(task("processing")).await;
    let notifications = Arc::new(Mutex::new(Vec::new()));
    let barrier = Arc::new(tokio::sync::Barrier::new(2));
    let progress_queue = queue.clone();
    let progress_events = notifications.clone();
    let progress_barrier = barrier.clone();
    let progress = tokio::spawn(async move {
        progress_barrier.wait().await;
        progress_queue
            .record_output("task-1", |_| {
                progress_events.lock().unwrap().push("working");
            })
            .await
    });
    barrier.wait().await;
    queue
        .release_and_notify("task-1", "completed", Default::default(), |_| {
            notifications.lock().unwrap().push("completed");
        })
        .await
        .unwrap();
    let progress = progress.await.unwrap();
    let events = notifications.lock().unwrap();
    match progress {
        Ok(_) => assert_eq!(*events, vec!["working", "completed"]),
        Err(error) => {
            assert_eq!(error, OutputError::NotActive);
            assert_eq!(*events, vec!["completed"]);
        }
    }
    drop(events);
    assert_eq!(queue.get("task-1").await.unwrap().status, "completed");
}
