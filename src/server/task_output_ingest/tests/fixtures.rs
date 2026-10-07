//! Process-local queue and bus fixtures; no authenticated worker claims.

use crate::{
    bus::{AgentBus, BusHandle},
    server::{KnativeTask, KnativeTaskQueue},
};
use std::sync::Arc;

pub(super) async fn setup(status: &str) -> (KnativeTaskQueue, Arc<AgentBus>, BusHandle) {
    let tasks = KnativeTaskQueue::new();
    tasks
        .push(KnativeTask {
            task_id: "task-1".into(),
            title: "title".into(),
            description: "description".into(),
            agent_type: "build".into(),
            model: Some("test-model".into()),
            metadata: Some(serde_json::json!({"source": "fixture"})),
            priority: 1,
            received_at: chrono::Utc::now(),
            status: status.into(),
            completion: Default::default(),
        })
        .await;
    let bus = AgentBus::new().into_arc();
    let reader = bus.handle("observer");
    (tasks, bus, reader)
}

pub(super) fn payload() -> super::super::request::TaskOutputPayload {
    super::super::request::TaskOutputPayload {
        worker_id: None,
        output: Some("progress".into()),
    }
}
