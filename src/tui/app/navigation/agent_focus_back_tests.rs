//! Tests for Shift+Tab backward spawned-agent focus cycling.

use crate::tui::app::navigation::cycle_agent_focus_back;
use crate::tui::app::state::App;
use crate::tui::app::state::agent_profile::SpawnedAgent;

fn mk(name: &str) -> SpawnedAgent {
    SpawnedAgent {
        name: name.to_string(),
        instructions: String::new(),
        parent: None,
        depth: 0,
        session: futures::executor::block_on(crate::session::Session::new()).unwrap(),
        model_id: Some("test/model".to_string()),
        is_processing: false,
    }
}

#[test]
fn shift_tab_cycles_agent_focus_backward() {
    let mut app = App::default();
    app.state.spawned_agents.insert("alpha".into(), mk("alpha"));
    app.state.spawned_agents.insert("beta".into(), mk("beta"));

    // None -> last agent (sorted: beta)
    cycle_agent_focus_back(&mut app);
    assert_eq!(app.state.active_spawned_agent.as_deref(), Some("beta"));

    // -> previous agent (alpha)
    cycle_agent_focus_back(&mut app);
    assert_eq!(app.state.active_spawned_agent.as_deref(), Some("alpha"));

    // -> back to main chat (None)
    cycle_agent_focus_back(&mut app);
    assert_eq!(app.state.active_spawned_agent, None);
}
