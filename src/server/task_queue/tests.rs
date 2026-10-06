//! Focused state-transition and concurrent-claim regressions.

mod race;
mod transitions;

use super::KnativeTask;

fn task(status: &str) -> KnativeTask {
    KnativeTask {
        task_id: "task-1".into(),
        title: "title".into(),
        description: "description".into(),
        agent_type: "build".into(),
        priority: 1,
        received_at: chrono::Utc::now(),
        status: status.into(),
    }
}
