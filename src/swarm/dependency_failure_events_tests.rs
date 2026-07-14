use super::report;
use crate::swarm::{SubTask, SubTaskResult};
use crate::tui::swarm_view::SwarmEvent;

#[test]
fn reports_blocked_task_as_failed() {
    let task = SubTask::new("dependent", "edit");
    let result = SubTaskResult {
        subtask_id: task.id.clone(),
        subagent_id: "blocked-agent".into(),
        success: false,
        result: String::new(),
        steps: 0,
        tool_calls: 0,
        execution_time_ms: 0,
        error: Some("prerequisite failed".into()),
        artifacts: Vec::new(),
        retry_count: 0,
    };
    let mut events = Vec::new();
    report(&[task], &[result], |event| events.push(event));
    assert!(matches!(
        events.first(),
        Some(SwarmEvent::SubTaskUpdate {
            status: crate::swarm::SubTaskStatus::Failed,
            ..
        })
    ));
    assert!(matches!(
        events.last(),
        Some(SwarmEvent::AgentComplete { success: false, .. })
    ));
}
