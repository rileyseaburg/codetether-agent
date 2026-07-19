//! Visibility decision for the main-chat agent bar.

use crate::tui::app::state::App;

pub(in crate::tui::ui::chat_view) fn visible(app: &App) -> bool {
    !app.state.spawned_agents.is_empty()
        || !app.state.swarm.subtasks.is_empty()
        || !super::tool_agents(app).is_empty()
}
