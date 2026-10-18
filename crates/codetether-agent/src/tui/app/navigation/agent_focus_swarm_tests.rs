use super::assemble;
use crate::swarm::SubTaskStatus;
use crate::tui::app::state::App;
use crate::tui::swarm_view::SubTaskInfo;

#[test]
fn swarm_agents_join_focus_ring_once() {
    let mut app = App::default();
    app.state.swarm.subtasks.push(swarm_task("worker-one"));
    app.state.swarm.subtasks.push(swarm_task("worker-one"));

    assert_eq!(assemble(&app, Vec::new()), vec!["worker-one"]);
}

fn swarm_task(agent_name: &str) -> SubTaskInfo {
    SubTaskInfo {
        id: agent_name.into(),
        name: "benchmark follow-up".into(),
        status: SubTaskStatus::Running,
        stage: 0,
        dependencies: Vec::new(),
        agent_name: Some(agent_name.into()),
        current_tool: None,
        steps: 0,
        max_steps: 10,
        tool_call_history: Vec::new(),
        messages: Vec::new(),
        output: None,
        error: None,
        started_at: None,
        elapsed_secs: None,
    }
}
