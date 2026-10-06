//! Exercise the real queue and in-process bus together (no external services).

use super::{super::service, events::assert_terminal_event, queue, request};
use crate::bus::AgentBus;

#[tokio::test]
async fn release_updates_queue_and_publishes_matching_terminal_event() {
    for status in ["completed", "success", "failed", "error", "unknown"] {
        let tasks = queue().await;
        let bus = AgentBus::new().into_arc();
        let mut observer = bus.handle("release-observer");
        let mut req = request(status);
        req.result = Some("partial work".into());
        req.error = Some("tool crashed".into());
        let success = matches!(status, "completed" | "success");
        let expected_status = if success { "completed" } else { "failed" };
        assert_eq!(
            service::release(&tasks, &bus, &req).await,
            Ok(expected_status)
        );
        assert_eq!(
            tasks.get(&req.task_id).await.unwrap().status,
            expected_status
        );
        let event = tokio::time::timeout(
            std::time::Duration::from_secs(1),
            observer.recv_topic("task.task-1"),
        )
        .await
        .unwrap()
        .unwrap();
        assert_terminal_event(event, success);
        assert_eq!(bus.recorder.recent(10, Some("task.task-1")).len(), 1);
        assert!(observer.try_recv().is_none());
    }
}
