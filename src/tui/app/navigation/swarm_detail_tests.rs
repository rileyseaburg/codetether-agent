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
        agent_name: None,
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
fn arrows_switch_agents_in_detail_mode() {
    let mut app = App::default();
    app.state.set_view_mode(ViewMode::Swarm);
    app.state.swarm.subtasks = vec![task("one"), task("two")];
    app.state.swarm.enter_detail();
    handle_down(&mut app, KeyModifiers::NONE);
    assert_eq!(app.state.swarm.selected_index, 1);
    handle_up(&mut app, KeyModifiers::NONE);
    assert_eq!(app.state.swarm.selected_index, 0);
}
