//! View transition into a newly spawned agent's live transcript.

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

/// Focus a child and open its live detail pane.
///
/// The dashboard remains available through the existing Esc binding.
pub(super) fn open(app: &mut App, name: &str) {
    app.state.active_spawned_agent = Some(name.to_string());
    app.state.set_view_mode(ViewMode::Subagents);
    app.state.subagent_detail_mode = true;
    app.state.subagent_detail_scroll = 0;
}

#[cfg(test)]
#[path = "spawn_agent_watch_tests.rs"]
mod tests;
