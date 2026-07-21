//! Chat-panel projection for the managed or remote agent selected by Tab.

use super::drawn_lines::DrawnLines;
use crate::tui::app::state::App;

pub(super) fn build(app: &mut App) -> Option<DrawnLines> {
    let name = app.state.active_spawned_agent.as_deref()?;
    if !known(app, name) {
        return None;
    }
    let lines = super::super::subagent_detail_lines::lines(&app.state);
    app.state.set_tool_preview_max_scroll(0);
    Some(DrawnLines::from_rebuild(lines))
}

fn known(app: &App, name: &str) -> bool {
    app.state.spawned_agents.contains_key(name)
        || app.state.session_id.as_deref().is_some_and(|parent| {
            crate::tool::agent::bridge::find_agent_tool_agent_for_parent(name, parent).is_some()
        })
}

#[cfg(test)]
#[path = "agent_lines_tests.rs"]
mod tests;
