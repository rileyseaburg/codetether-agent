//! Process-local queue fixtures for polling receipt contracts.

use crate::server::{KnativeTask, task_queue::TaskCompletion};

pub(super) fn task() -> KnativeTask {
    KnativeTask {
        task_id: "task-1".into(),
        title: "receipt".into(),
        description: String::new(),
        agent_type: "build".into(),
        model: None,
        metadata: None,
        priority: 0,
        received_at: chrono::Utc::now(),
        status: "working".into(),
        completion: Default::default(),
    }
}

pub(super) fn receipt() -> TaskCompletion {
    TaskCompletion {
        result: Some("result".into()),
        error: Some("diagnostic".into()),
        session_id: Some("session-1".into()),
        diagnostics: Some(serde_json::json!({"attempt": 1})),
    }
}
