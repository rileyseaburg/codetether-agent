//! A missing task must not create a phantom terminal notification.

use super::{super::service, request};
use crate::{
    bus::AgentBus,
    server::{KnativeTaskQueue, task_queue::ReleaseError},
};

#[tokio::test]
async fn missing_task_emits_no_terminal_event() {
    let tasks = KnativeTaskQueue::new();
    let bus = AgentBus::new().into_arc();
    let mut observer = bus.handle("release-observer");
    assert_eq!(
        service::release(&tasks, &bus, &request("failed")).await,
        Err(ReleaseError::NotFound)
    );
    assert!(tasks.list().await.is_empty());
    assert!(bus.recorder.recent(10, Some("task.task-1")).is_empty());
    assert!(observer.try_recv().is_none());
}
