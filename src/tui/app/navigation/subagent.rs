//! Navigation behavior for the managed-agent dashboard and detail pane.

use crate::tui::app::state::App;

pub(super) fn escape(app: &mut App) {
    if app.state.subagent_detail_mode {
        app.state.subagent_detail_mode = false;
        app.state.subagent_detail_scroll = 0;
        app.state.status = "Agent dashboard".to_string();
    } else {
        crate::tui::app::session_sync::return_to_chat(app);
    }
}

pub(super) fn up(app: &mut App) {
    if app.state.subagent_detail_mode {
        app.state.subagent_detail_scroll = app.state.subagent_detail_scroll.saturating_sub(1);
    } else {
        super::cycle_agent_focus_back(app);
    }
}

pub(super) fn down(app: &mut App) {
    if app.state.subagent_detail_mode {
        app.state.subagent_detail_scroll = app.state.subagent_detail_scroll.saturating_add(1);
    } else {
        super::cycle_agent_focus(app);
    }
}

pub(super) fn page_up(app: &mut App) {
    app.state.subagent_detail_scroll = app.state.subagent_detail_scroll.saturating_sub(10);
}

pub(super) fn page_down(app: &mut App) {
    app.state.subagent_detail_scroll = app.state.subagent_detail_scroll.saturating_add(10);
}
