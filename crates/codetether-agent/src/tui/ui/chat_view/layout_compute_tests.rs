use super::compute_chat_chunks;
use crate::swarm::SubTaskStatus;
use crate::tui::app::state::App;
use crate::tui::swarm_view::SubTaskInfo;
use ratatui::layout::Rect;

#[test]
fn direct_swarm_reserves_the_main_agent_bar() {
    let mut app = App::default();
    app.state.swarm.subtasks.push(SubTaskInfo {
        id: "review".into(),
        name: "Review".into(),
        status: SubTaskStatus::Running,
        stage: 0,
        dependencies: Vec::new(),
        agent_name: Some("agent-review".into()),
        current_tool: None,
        steps: 0,
        max_steps: 20,
        tool_call_history: Vec::new(),
        messages: Vec::new(),
        output: None,
        error: None,
        started_at: None,
        elapsed_secs: None,
    });
    let chunks = compute_chat_chunks(Rect::new(0, 0, 100, 30), &app);
    assert_eq!(chunks.agent_bar.height, 1);
}
