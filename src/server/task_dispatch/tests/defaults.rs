//! Compatibility coverage for omitted optional dispatch fields.

use super::super::{contract::DispatchTaskRequest, service::enqueue};
use crate::{bus::AgentBus, server::KnativeTaskQueue};

#[tokio::test]
async fn dispatch_defaults_remain_compatible() {
    let queue = KnativeTaskQueue::new();
    let request: DispatchTaskRequest = serde_json::from_value(serde_json::json!({
        "title": "Default task", "description": "Minimal supported request"
    }))
    .unwrap();
    let bus = AgentBus::with_capacity(8).into_arc();
    let receipt = enqueue(&queue, &bus, request).await;
    let task = queue.get(&receipt.task_id).await.unwrap();
    assert_eq!(task.agent_type, "build");
    assert_eq!(task.priority, 0);
    assert_eq!(task.model, None);
    assert_eq!(task.metadata, None);
    let mut legacy = serde_json::to_value(&task).unwrap();
    legacy.as_object_mut().unwrap().remove("model");
    legacy.as_object_mut().unwrap().remove("metadata");
    assert!(serde_json::from_value::<crate::server::KnativeTask>(legacy).is_ok());
}
