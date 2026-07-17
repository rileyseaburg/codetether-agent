//! Shift+Tab backward cycling of the active agent focus.

use super::{agent_names, child, detail, set_focus};
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

/// Cycle the applicable focus ring backward by one slot.
pub fn cycle_agent_focus_back(app: &mut App) {
    if app.state.view_mode == ViewMode::Subagents {
        if app.state.subagent_detail_mode {
            detail::cycle_back(app);
        } else {
            child::previous(app);
        }
        return;
    }
    let names = agent_names(app);
    if names.is_empty() {
        app.state.status = "No spawned agents. Use /spawn <name> to create one.".to_string();
        return;
    }
    let previous = match &app.state.active_spawned_agent {
        None => Some(names[names.len() - 1].clone()),
        Some(current) => match names.iter().position(|name| name == current) {
            Some(0) | None => None,
            Some(index) => Some(names[index - 1].clone()),
        },
    };
    set_focus(app, previous);
}
