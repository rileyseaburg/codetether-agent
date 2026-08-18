use crate::a2a::types::TaskState;
use crate::bus::BusMessage;

use super::ActiveTasks;
use super::observe::MAX_ACTIVE_TASKS;

fn update(task_id: &str, state: TaskState) -> BusMessage {
    BusMessage::TaskUpdate {
        task_id: task_id.to_string(),
        state,
        message: None,
    }
}

#[test]
fn tracks_active_and_clears_on_terminal() {
    let mut tasks = ActiveTasks::default();
    tasks.observe(&update("a", TaskState::Working));
    tasks.observe(&update("b", TaskState::Submitted));
    assert_eq!(tasks.count(), 2);
    tasks.observe(&update("a", TaskState::Completed));
    assert_eq!(tasks.count(), 1);
    tasks.observe(&update("b", TaskState::Failed));
    assert_eq!(tasks.count(), 0);
}

#[test]
fn ignores_non_task_messages() {
    let mut tasks = ActiveTasks::default();
    tasks.observe(&BusMessage::AgentShutdown {
        agent_id: "x".into(),
    });
    assert_eq!(tasks.count(), 0);
}

#[test]
fn missing_terminal_events_cannot_grow_forever() {
    let mut tasks = ActiveTasks::default();
    for index in 0..MAX_ACTIVE_TASKS + 10 {
        tasks.observe(&update(&index.to_string(), TaskState::Working));
    }
    assert_eq!(tasks.count(), MAX_ACTIVE_TASKS);
}
