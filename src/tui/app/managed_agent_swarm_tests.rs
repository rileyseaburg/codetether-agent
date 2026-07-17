use super::*;
use crate::swarm::SubTaskStatus;
use crate::tui::swarm_view::SubTaskInfo;

#[test]
fn prefix_focus_opens_matching_swarm_worker() {
    let mut app = App::default();
    app.state.swarm.subtasks.push(SubTaskInfo {
        id: "one".into(),
        name: "inspect".into(),
        status: SubTaskStatus::Running,
        stage: 0,
        dependencies: vec![],
        agent_name: Some("agent-abc123".into()),
        current_tool: None,
        steps: 0,
        max_steps: 10,
        tool_call_history: vec![],
        messages: vec![],
        output: None,
        error: None,
        started_at: None,
        elapsed_secs: None,
    });
    assert!(focus(&mut app, "agent-abc"));
    assert_eq!(app.state.view_mode, ViewMode::Swarm);
    assert!(app.state.swarm.detail_mode);
}
