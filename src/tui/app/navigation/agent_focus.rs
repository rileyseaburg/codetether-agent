//! Tab-key cycling of the active spawned-agent focus.
//!
//! The status bar advertises `Tab → Switch Agent`. This rotates
//! [`App::active_spawned_agent`] through the main chat (`None`) and
//! each spawned agent in insertion order, so plain chat input is
//! routed to the focused teammate.

use crate::tui::app::state::App;

/// Handle the Tab key: accept a slash suggestion if one is active,
/// otherwise cycle the active spawned-agent focus.
pub fn handle_tab(app: &mut App) {
    if app.state.apply_selected_slash_suggestion() {
        app.state.status = "Command autocompleted".to_string();
    } else {
        cycle_agent_focus(app);
    }
}

/// Cycle the active spawned-agent focus forward by one slot.
///
/// The cycle is: main chat (`None`) → agent 1 → agent 2 → … → main chat.
/// When no agents are spawned this is a no-op with a hint status.
pub fn cycle_agent_focus(app: &mut App) {
    let names: Vec<String> = app.state.spawned_agents.keys().cloned().collect();
    if names.is_empty() {
        app.state.status = "No spawned agents. Use /spawn <name> to create one.".to_string();
        return;
    }

    let next = match &app.state.active_spawned_agent {
        None => Some(names[0].clone()),
        Some(current) => match names.iter().position(|n| n == current) {
            Some(idx) if idx + 1 < names.len() => Some(names[idx + 1].clone()),
            _ => None,
        },
    };

    app.state.status = match &next {
        Some(name) => format!("Focused agent: {name}"),
        None => "Focused main chat".to_string(),
    };
    app.state.active_spawned_agent = next;
}
