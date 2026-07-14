use super::*;

#[path = "agent_focus_back_tests.rs"]
mod agent_focus_back_tests;
#[path = "subagent_detail_tests.rs"]
mod subagent_detail_tests;
#[path = "swarm_detail_tests.rs"]
mod swarm_detail_tests;

#[test]
fn help_overlay_consumes_chat_arrow_navigation() {
    let mut app = App::default();
    app.state.show_help = true;
    app.state.set_view_mode(ViewMode::Chat);
    app.state.set_chat_max_scroll(25);
    app.state.scroll_to_bottom();

    handle_down(&mut app, KeyModifiers::NONE);

    assert_eq!(app.state.help_scroll.offset, 1);
    assert_eq!(app.state.chat_scroll, 1_000_000);
}

#[test]
fn tab_cycles_agent_focus_through_main_and_back() {
    use crate::tui::app::state::agent_profile::SpawnedAgent;
    let mut app = App::default();
    let mk = |name: &str| SpawnedAgent {
        name: name.to_string(),
        instructions: String::new(),
        parent: None,
        depth: 0,
        session: futures::executor::block_on(crate::session::Session::new()).unwrap(),
        model_id: Some("test/model".to_string()),
        is_processing: false,
    };
    app.state.spawned_agents.insert("alpha".into(), mk("alpha"));
    app.state.spawned_agents.insert("beta".into(), mk("beta"));

    // None -> first agent
    handle_tab(&mut app);
    let first = app.state.active_spawned_agent.clone().unwrap();
    assert!(first == "alpha" || first == "beta");

    // -> second agent
    handle_tab(&mut app);
    assert!(app.state.active_spawned_agent.is_some());

    // -> back to main chat (None)
    handle_tab(&mut app);
    assert_eq!(app.state.active_spawned_agent, None);
}
