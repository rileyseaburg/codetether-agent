//! Shared fixtures for release outcome and queue/event regression coverage.

#[path = "tests/events.rs"]
mod events;
#[path = "tests/guards.rs"]
mod guards;
#[path = "tests/http_error.rs"]
mod http_error;
#[path = "tests/lifecycle.rs"]
mod lifecycle;
#[path = "tests/messages.rs"]
mod messages;
#[path = "tests/missing.rs"]
mod missing;
#[path = "tests/normalization.rs"]
mod normalization;
#[path = "tests/race.rs"]
mod race;

use super::ReleaseRequest;
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
            priority: 1,
            received_at: chrono::Utc::now(),
            status: "working".into(),
        })
        .await;
    tasks
}
