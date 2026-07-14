use super::*;
use crate::swarm::SubTaskStatus;
use crate::tui::swarm_view::SubTaskInfo;

fn task() -> SubTaskInfo {
    SubTaskInfo {
        id: "task-1".into(),
        name: "inspect".into(),
        status: SubTaskStatus::Running,
        stage: 0,
        dependencies: vec![],
        agent_name: Some("agent-1".into()),
        current_tool: None,
        steps: 0,
        max_steps: 10,
        tool_call_history: vec![],
        messages: vec![],
        output: None,
        error: None,
        started_at: None,
        elapsed_secs: None,
    }
}

#[test]
fn agents_enter_opens_swarm_detail() {
    let mut app = App::default();
    app.state.swarm.subtasks.push(task());
    dispatch_subagents_enter(&mut app);
    assert_eq!(app.state.view_mode, ViewMode::Swarm);
    assert!(app.state.swarm.detail_mode);
}

#[test]
fn agents_enter_reports_empty_swarm() {
    let mut app = App::default();
    dispatch_subagents_enter(&mut app);
    assert_eq!(app.state.status, "No swarm agents to inspect");
}
