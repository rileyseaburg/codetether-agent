//! Regression coverage for the process-local dispatch contract.

use super::{contract::DispatchTaskRequest, service::enqueue};
use crate::{bus::AgentBus, server::KnativeTaskQueue};

#[tokio::test]
async fn dispatch_preserves_execution_configuration_before_notifying() {
    let queue = KnativeTaskQueue::new();
    let bus = AgentBus::with_capacity(8).into_arc();
    let mut receiver = bus.handle("dispatch-test").into_receiver();
    let metadata = serde_json::json!({"codebase_id": "workspace-1"});
    let request = DispatchTaskRequest {
        title: "Configured task".into(),
        description: "Keep execution settings".into(),
        agent_type: Some("review".into()),
        model: Some("provider/model".into()),
        priority: Some(7),
        metadata: Some(metadata.clone()),
    };
    let receipt = enqueue(&queue, &bus, request).await;
    let task = queue.get(&receipt.task_id).await.unwrap();
    assert_eq!(task.model.as_deref(), Some("provider/model"));
    assert_eq!(task.metadata, Some(metadata));
    assert_eq!(task.agent_type, "review");
    assert_eq!(task.priority, 7);
    assert_eq!(task.status, "pending");
    assert_eq!(receipt.status, "pending");
    assert_eq!(receipt.dispatch_mode, "local_queue");
    assert!(!receipt.dispatched_via_knative);
    assert!(!receipt.durable);
    let notification = receiver.try_recv().unwrap();
    assert_eq!(notification.topic, format!("task.{}", receipt.task_id));
    assert!(matches!(notification.message,
        crate::bus::BusMessage::TaskUpdate {
            task_id, state: crate::a2a::types::TaskState::Submitted, ..
        } if task_id == receipt.task_id));
    assert!(queue.get(&receipt.task_id).await.is_some());
}

#[path = "tests/defaults.rs"]
mod defaults;
#[path = "tests/receipt.rs"]
mod receipt;
