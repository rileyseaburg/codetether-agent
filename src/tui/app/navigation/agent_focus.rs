//! Tab-key cycling of the active spawned-agent focus.
//!
//! The status bar advertises `Tab → Switch Agent`. This rotates
//! [`App::active_spawned_agent`] through the main chat (`None`) and
//! each spawned agent in insertion order, so plain chat input is
//! routed to the focused teammate.

use crate::tui::app::state::App;

#[path = "agent_focus_back.rs"]
mod back;
#[path = "agent_focus_detail.rs"]
mod detail;
#[path = "agent_focus_names.rs"]
mod names;
pub use back::cycle_agent_focus_back;
use names::agent_names;

/// Handle the Tab key: accept a slash suggestion if one is active,
/// otherwise cycle the active spawned-agent focus.
pub fn handle_tab(app: &mut App) {
    if app.state.apply_selected_slash_suggestion() {
        app.state.status = "Command autocompleted".to_string();
    } else if app.state.view_mode == crate::tui::models::ViewMode::Subagents
        && app.state.subagent_detail_mode
    {
        detail::cycle(app);
    } else {
        cycle_agent_focus(app);
        if app.state.view_mode == crate::tui::models::ViewMode::Subagents {
            app.state.subagent_detail_scroll = 0;
        }
    }
}

/// Apply a focus selection and update the status line.
pub(super) fn set_focus(app: &mut App, next: Option<String>) {
    app.state.status = match &next {
        Some(name) => format!("Focused agent: {name}"),
        None => "Focused main chat".to_string(),
    };
    app.state.active_spawned_agent = next;
}

/// Cycle the active spawned-agent focus forward by one slot.
///
/// The cycle is: main chat (`None`) → agent 1 → agent 2 → … → main chat.
/// When no agents are spawned this is a no-op with a hint status.
pub fn cycle_agent_focus(app: &mut App) {
    let names = agent_names(app);
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
    set_focus(app, next);
}
