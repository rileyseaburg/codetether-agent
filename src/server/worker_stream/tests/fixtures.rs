//! Queue and bus fixtures without external worker dependencies.

use crate::a2a::types::TaskState;
use crate::bus::{BusEnvelope, BusMessage};
use crate::server::KnativeTask;

pub(super) fn task() -> KnativeTask {
    KnativeTask {
        task_id: "configured-task".into(),
        title: "Review".into(),
        description: "Keep execution configuration".into(),
        agent_type: "review".into(),
        model: Some("provider/model".into()),
        metadata: Some(serde_json::json!({"codebase_id": "workspace-1"})),
        priority: 7,
        received_at: chrono::Utc::now(),
        status: "pending".into(),
        completion: Default::default(),
    }
}

pub(super) fn notification(task_id: &str) -> BusEnvelope {
    BusEnvelope {
        id: uuid::Uuid::new_v4().to_string(),
        topic: format!("task.{task_id}"),
        sender_id: "dispatch-test".into(),
        correlation_id: None,
        timestamp: chrono::Utc::now(),
        message: BusMessage::TaskUpdate {
            task_id: task_id.into(),
            state: TaskState::Submitted,
            message: None,
        },
    }
}
