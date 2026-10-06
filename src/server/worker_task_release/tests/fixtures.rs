//! Active task and release-request fixtures shared by the release regressions.

use super::super::ReleaseRequest;
use crate::server::{KnativeTask, KnativeTaskQueue};

pub(super) fn request(status: &str) -> ReleaseRequest {
    ReleaseRequest {
        task_id: "task-1".into(),
        status: status.into(),
        result: None,
        error: None,
        session_id: None,
        diagnostics: None,
    }
}

pub(super) async fn queue() -> KnativeTaskQueue {
    let tasks = KnativeTaskQueue::new();
    tasks
        .push(KnativeTask {
            task_id: "task-1".into(),
            title: "release regression".into(),
            description: String::new(),
            agent_type: "general".into(),
            model: None,
            metadata: None,
            priority: 1,
            received_at: chrono::Utc::now(),
            status: "working".into(),
            completion: Default::default(),
        })
        .await;
    tasks
}
