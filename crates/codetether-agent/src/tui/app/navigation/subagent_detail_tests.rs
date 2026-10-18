use super::*;

#[test]
fn escape_returns_from_child_detail_to_agent_dashboard() {
    let mut app = App::default();
    crate::tui::app::state::view::transition(&mut app.state).set_view_mode(ViewMode::Subagents);
    app.state.agent_tree.detail_mode = true;
    app.state.agent_tree.detail_scroll = 12;
    handle_escape(&mut app);
    assert_eq!(app.state.view.mode, ViewMode::Subagents);
    assert!(!app.state.agent_tree.detail_mode);
    assert_eq!(app.state.agent_tree.detail_scroll, 0);
}

#[test]
fn detail_arrows_scroll_transcript() {
    let mut app = App::default();
    crate::tui::app::state::view::transition(&mut app.state).set_view_mode(ViewMode::Subagents);
    app.state.agent_tree.detail_mode = true;
    handle_down(&mut app, KeyModifiers::NONE);
    assert_eq!(app.state.agent_tree.detail_scroll, 1);
    handle_up(&mut app, KeyModifiers::NONE);
    assert_eq!(app.state.agent_tree.detail_scroll, 0);
}
