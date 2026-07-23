use super::*;
use crate::swarm::SubTaskStatus;
use crate::tui::swarm_view::SubTaskInfo;

fn task(id: &str) -> SubTaskInfo {
    SubTaskInfo {
        id: id.into(),
        name: id.into(),
        status: SubTaskStatus::Running,
        stage: 0,
        dependencies: vec![],
        agent_name: Some(format!("agent-{id}")),
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
fn tab_and_backtab_navigate_swarm_workers() {
    let mut app = App::default();
    app.state.set_view_mode(ViewMode::Swarm);
    app.state.swarm.subtasks = vec![task("one"), task("two")];
    dispatch(&mut app, KeyCode::Tab);
    assert_eq!(app.state.swarm.selected_index, 1);
    assert_eq!(app.state.view_mode, ViewMode::Swarm);
    dispatch(&mut app, KeyCode::BackTab);
    assert_eq!(app.state.swarm.selected_index, 0);
    dispatch(&mut app, KeyCode::BackTab);
    assert_eq!(app.state.swarm.selected_index, 1);
    dispatch(&mut app, KeyCode::Tab);
    assert_eq!(app.state.swarm.selected_index, 0);
}
