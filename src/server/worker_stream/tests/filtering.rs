//! Notifications cannot substitute for authoritative queue state.

use super::{super::delivery, fixtures};
use crate::{a2a::types::TaskState, bus::BusMessage, server::KnativeTaskQueue};

#[tokio::test]
async fn live_filters_unrelated_missing_and_claimed_tasks() {
    let queue = KnativeTaskQueue::new();
    let task = fixtures::task();
    queue.push(task.clone()).await;
    let mut wrong_topic = fixtures::notification(&task.task_id);
    wrong_topic.topic.push_str("-other");
    assert!(delivery::live(&queue, wrong_topic).await.is_none());
    let mut unrelated = fixtures::notification(&task.task_id);
    unrelated.message = BusMessage::AgentShutdown {
        agent_id: "other".into(),
    };
    assert!(delivery::live(&queue, unrelated).await.is_none());
    let mut working = fixtures::notification(&task.task_id);
    working.message = BusMessage::TaskUpdate {
        task_id: task.task_id.clone(),
        state: TaskState::Working,
        message: None,
    };
    assert!(delivery::live(&queue, working).await.is_none());
    assert!(
        delivery::live(&queue, fixtures::notification("missing"))
            .await
            .is_none()
    );
    assert!(
        delivery::live(&queue, fixtures::notification(&task.task_id))
            .await
            .is_some()
    );
    for status in ["queued", "working", "completed", "failed", "cancelled"] {
        assert!(queue.update_status(&task.task_id, status).await);
        assert_eq!(
            delivery::live(&queue, fixtures::notification(&task.task_id))
                .await
                .is_some(),
            status == "queued",
        );
    }
}
