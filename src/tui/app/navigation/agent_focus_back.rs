//! Shift+Tab backward cycling of the active spawned-agent focus.
//!
//! Companion to [`super::cycle_agent_focus`]; rotates the focus ring in
//! the opposite direction so Shift+Tab steps back through teammates.

use super::{agent_names, set_focus};
use crate::tui::app::state::App;

/// Cycle the active spawned-agent focus backward by one slot.
///
/// The cycle is: main chat (`None`) → last agent → … → agent 1 → main chat.
/// When no agents are spawned this is a no-op with a hint status.
pub fn cycle_agent_focus_back(app: &mut App) {
    let names = agent_names(app);
    if names.is_empty() {
        app.state.status = "No spawned agents. Use /spawn <name> to create one.".to_string();
        return;
    }
    let last = names.len() - 1;
    let prev = match &app.state.active_spawned_agent {
        None => Some(names[last].clone()),
        Some(current) => match names.iter().position(|n| n == current) {
            Some(0) | None => None,
            Some(idx) => Some(names[idx - 1].clone()),
        },
    };
    set_focus(app, prev);
}
