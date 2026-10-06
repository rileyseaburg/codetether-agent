//! Rejected releases must preserve state and must not notify observers.

use super::{super::service, queue, request};
use crate::{bus::AgentBus, server::task_queue::ReleaseError};

#[tokio::test]
async fn inactive_tasks_cannot_be_released() {
    for status in ["pending", "queued", "completed", "failed", "cancelled"] {
        let tasks = queue().await;
        assert!(tasks.update_status("task-1", status).await);
        let before = tasks.get("task-1").await.unwrap();
        let bus = AgentBus::new().into_arc();
        let mut observer = bus.handle("release-observer");
        assert_eq!(
            service::release(&tasks, &bus, &request("completed")).await,
            Err(ReleaseError::NotActive)
        );
        let after = tasks.get("task-1").await.unwrap();
        assert_eq!(after.status, before.status);
        assert_eq!(after.description, before.description);
        assert!(bus.recorder.recent(10, Some("task.task-1")).is_empty());
        assert!(observer.try_recv().is_none());
    }
}

#[tokio::test]
async fn repeated_release_cannot_overwrite_the_first_outcome() {
    for status in ["processing", "working"] {
        let tasks = queue().await;
        assert!(tasks.update_status("task-1", status).await);
        let bus = AgentBus::new().into_arc();
        assert_eq!(
            service::release(&tasks, &bus, &request("completed")).await,
            Ok("completed")
        );
        assert_eq!(
            service::release(&tasks, &bus, &request("failed")).await,
            Err(ReleaseError::NotActive)
        );
        assert_eq!(tasks.get("task-1").await.unwrap().status, "completed");
        assert_eq!(bus.recorder.recent(10, Some("task.task-1")).len(), 1);
    }
}

#[tokio::test]
async fn invalid_terminal_status_cannot_mutate_an_active_task() {
    let tasks = queue().await;
    assert!(matches!(
        tasks.release("task-1", "working").await,
        Err(ReleaseError::InvalidStatus)
    ));
    assert_eq!(tasks.get("task-1").await.unwrap().status, "working");
}
