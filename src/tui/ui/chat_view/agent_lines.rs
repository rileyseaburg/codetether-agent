//! Chat-panel projection for the managed or remote agent selected by Tab.

use super::drawn_lines::DrawnLines;
use crate::tui::app::state::App;
use crate::tui::message_formatter::MessageFormatter;

pub(super) fn build(
    app: &mut App,
    content_width: usize,
    formatter: &MessageFormatter,
) -> Option<DrawnLines> {
    let name = app.state.active_spawned_agent.as_deref()?;
    if !known(app, name) {
        return None;
    }
    let width = content_width.saturating_sub(2);
    let lines = super::super::subagent_detail_lines::lines(&app.state)
        .into_iter()
        .flat_map(|line| formatter.wrap_line(line.spans, width))
        .collect();
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
