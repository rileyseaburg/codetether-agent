use super::*;
use crate::tui::app::state::SpawnedAgent;

fn child(name: &str) -> SpawnedAgent {
    SpawnedAgent {
        name: name.into(),
        instructions: String::new(),
        parent: None,
        depth: 0,
        session: futures::executor::block_on(crate::session::Session::new()).unwrap(),
        model_id: None,
        is_processing: false,
    }
}

#[test]
fn detail_cycle_wraps_without_selecting_main_chat() {
    let mut app = App::default();
    app.state
        .spawned_agents
        .insert("alpha".into(), child("alpha"));
    app.state.active_spawned_agent = Some("alpha".into());
    cycle(&mut app);
    assert_eq!(app.state.active_spawned_agent.as_deref(), Some("alpha"));
}
