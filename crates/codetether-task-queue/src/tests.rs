//! Focused state-transition and concurrent-claim regressions.

mod completion;
mod output;
mod output_race;
mod pending;
mod race;
mod rejected_completion;
mod rejected_output;
mod serialization;
mod transitions;

use super::KnativeTask;

fn task(status: &str) -> KnativeTask {
    KnativeTask {
        task_id: "task-1".into(),
        title: "title".into(),
        description: "description".into(),
        agent_type: "build".into(),
        model: None,
        metadata: None,
        priority: 1,
        received_at: chrono::Utc::now(),
        status: status.into(),
        completion: Default::default(),
    }
}
