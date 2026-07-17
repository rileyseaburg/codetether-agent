//! Main-chat-inclusive agent focus cycling.

use crate::tui::app::state::App;

/// Cycle from main chat through every child and back to main chat.
pub fn cycle_agent_focus(app: &mut App) {
    let names = super::agent_names(app);
    if names.is_empty() {
        app.state.status = "No spawned agents. Use /spawn <name> to create one.".to_string();
        return;
    }
    let next = match &app.state.active_spawned_agent {
        None => Some(names[0].clone()),
        Some(current) => names
            .iter()
            .position(|name| name == current)
            .and_then(|index| names.get(index + 1).cloned()),
    };
    super::set_focus(app, next);
}
