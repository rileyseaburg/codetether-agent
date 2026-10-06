//! Competing releases have one winner and one terminal notification.

use super::{super::service, contender, queue, race_assertions};
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
            let req = contender::receipt(index);
            barrier.wait().await;
            service::release(&tasks, &bus, &req)
                .await
                .map(|status| (status, req))
        });
    }
    let mut winner = None;
    let mut conflicts = 0;
    while let Some(result) = releases.join_next().await {
        match result.unwrap() {
            Ok(receipt) => assert!(winner.replace(receipt).is_none()),
            Err(ReleaseError::NotActive) => conflicts += 1,
            Err(error) => panic!("unexpected release rejection: {error}"),
        }
    }
    assert_eq!(conflicts, 15);
    let (winner, receipt) = winner.unwrap();
    race_assertions::receipt_matches(tasks.get("task-1").await.unwrap(), &winner, &receipt);
    race_assertions::event_matches(observer.try_recv().unwrap(), &winner, &receipt);
    assert!(observer.try_recv().is_none());
    assert_eq!(bus.recorder.recent(100, Some("task.task-1")).len(), 1);
}
