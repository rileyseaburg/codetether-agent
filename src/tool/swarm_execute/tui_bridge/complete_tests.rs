use super::super::{Observer, drain, started};
use super::test_support::{LOCK, task};
use crate::swarm::SubTaskStatus;
use crate::tool::swarm_execute::{model_selection::ModelSelection, task_result::TaskResult};
use crate::tui::swarm_view::SwarmViewState;

#[test]
fn direct_swarm_appears_in_tui_state() {
    let _guard = LOCK.lock().unwrap();
    let mut view = SwarmViewState::default();
    drain(&mut view);
    let mut observer = Observer::begin(&[task("Review")], 20);
    assert!(drain(&mut view));
    assert_eq!(view.task, "Direct swarm: Review");
    let events = observer.sender();
    started(&events, "task-1", None);
    observer.complete(&[result()]);
    assert!(drain(&mut view));
    assert!(view.complete);
    assert_eq!(view.subtasks[0].status, SubTaskStatus::Completed);
    assert_eq!(view.subtasks[0].output.as_deref(), Some("Looks good"));
}

fn result() -> TaskResult {
    TaskResult {
        task_id: "task-1".into(),
        task_name: "Review".into(),
        success: true,
        output: "Looks good".into(),
        error: None,
        steps: 2,
        tool_calls: 1,
        model: ModelSelection {
            requested_model: None,
            resolved_provider: "test".into(),
            resolved_model: "test-model".into(),
        },
    }
}
