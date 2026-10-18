use super::*;
use crate::tui::app::state::SpawnedAgent;

fn managed(name: &str) -> SpawnedAgent {
    SpawnedAgent {
        name: name.into(),
        instructions: "inspect".into(),
        parent: None,
        depth: 0,
        session: futures::executor::block_on(crate::session::Session::new()).unwrap(),
        model_id: Some("openai-codex/test".into()),
        is_processing: true,
    }
}

#[test]
fn enter_opens_managed_agent_detail_before_swarm() {
    let mut app = App::default();
    crate::tui::app::state::view::transition(&mut app.state).set_view_mode(ViewMode::Subagents);
    app.state
        .agent_tree
        .spawned
        .insert("observer".into(), managed("observer"));
    dispatch(&mut app);
    assert_eq!(app.state.view.mode, ViewMode::Subagents);
    assert_eq!(app.state.agent_tree.active.as_deref(), Some("observer"));
    assert!(app.state.agent_tree.detail_mode);
}

#[test]
fn enter_reports_empty_dashboard() {
    let mut app = App::default();
    dispatch(&mut app);
    assert_eq!(app.state.status, "No agents to inspect");
}
