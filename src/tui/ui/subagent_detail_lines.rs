//! Live transcript rows for the focused managed or tool-spawned agent.

use ratatui::style::Stylize;
use ratatui::text::Line;

use crate::tui::app::state::AppState;

pub(super) fn lines(state: &AppState) -> Vec<Line<'static>> {
    let Some(name) = state.active_spawned_agent.as_deref() else {
        return vec![Line::from("No managed agent selected.".dim())];
    };
    if let Some(agent) = state.spawned_agents.get(name) {
        return super::subagent_detail_managed::lines(agent);
    }
    super::subagent_detail_tool::lines(state, name)
}

#[cfg(test)]
#[path = "subagent_detail_lines_tests.rs"]
mod tests;
