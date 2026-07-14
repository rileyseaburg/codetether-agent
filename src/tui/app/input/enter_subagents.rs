//! Enter-key handling for the unified agent dashboard.

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

pub(super) fn dispatch(app: &mut App) {
    discard_stale_focus(app);
    if let Some(name) = app.state.active_spawned_agent.clone() {
        app.state.subagent_detail_mode = true;
        app.state.subagent_detail_scroll = 0;
        app.state.status = format!("Agent @{name} detail");
    } else if !app.state.swarm.subtasks.is_empty() {
        app.state.set_view_mode(ViewMode::Swarm);
        app.state.swarm.enter_detail();
        app.state.status = "Swarm agent detail".to_string();
    } else {
        crate::tui::app::navigation::cycle_agent_focus(app);
        if app.state.active_spawned_agent.is_some() {
            dispatch(app);
        } else {
            app.state.status = "No agents to inspect".to_string();
        }
    }
}

fn discard_stale_focus(app: &mut App) {
    let Some(name) = app.state.active_spawned_agent.as_deref() else {
        return;
    };
    let local = app.state.spawned_agents.contains_key(name);
    let owned_tool = app.state.session_id.as_deref().is_some_and(|parent| {
        crate::tool::agent::bridge::find_agent_tool_agent_for_parent(name, parent).is_some()
    });
    if !local && !owned_tool {
        app.state.active_spawned_agent = None;
    }
}

#[cfg(test)]
#[path = "enter_subagents_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "enter_swarm_tests.rs"]
mod swarm_tests;
