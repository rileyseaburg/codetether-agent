//! Competing releases have one winner and one terminal notification.

use super::{super::service, events::assert_terminal_event, queue, request};
use crate::{bus::AgentBus, server::task_queue::ReleaseError};
use std::sync::Arc;
use tokio::{sync::Barrier, task::JoinSet};

#[tokio::test]
async fn sixteen_competing_releases_emit_one_terminal_event() {
    let tasks = queue().await;
    let bus = AgentBus::new().into_arc();
    let mut observer = bus.handle("release-observer");
    let barrier = Arc::new(Barrier::new(16));
    let mut releases = JoinSet::new();
    for index in 0..16 {
        let tasks = tasks.clone();
        let bus = bus.clone();
        let barrier = barrier.clone();
        releases.spawn(async move {
            let mut req = request(if index % 2 == 0 {
                "completed"
            } else {
                "failed"
            });
            req.result = Some("partial work".into());
            req.error = Some("tool crashed".into());
            barrier.wait().await;
            service::release(&tasks, &bus, &req).await
        });
    }
    let mut winner = None;
    let mut conflicts = 0;
    while let Some(result) = releases.join_next().await {
        match result.unwrap() {
            Ok(status) => assert!(winner.replace(status).is_none()),
            Err(ReleaseError::NotActive) => conflicts += 1,
            Err(error) => panic!("unexpected release rejection: {error}"),
        }
    }
    assert_eq!(conflicts, 15);
    let winner = winner.unwrap();
    assert_eq!(tasks.get("task-1").await.unwrap().status, winner);
    assert_terminal_event(observer.try_recv().unwrap(), winner == "completed");
    assert!(observer.try_recv().is_none());
    assert_eq!(bus.recorder.recent(100, Some("task.task-1")).len(), 1);
}
