//! View transition into a newly spawned agent's live transcript.

use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

/// Focus a child and open its live detail pane.
///
/// The dashboard remains available through the existing Esc binding.
pub(super) fn open(app: &mut App, name: &str) {
    app.state.agent_tree.active = Some(name.to_string());
    crate::tui::app::state::view::transition(&mut app.state).set_view_mode(ViewMode::Subagents);
    app.state.agent_tree.detail_mode = true;
    app.state.agent_tree.detail_scroll = 0;
}

#[cfg(test)]
#[path = "spawn_agent_watch_tests.rs"]
mod tests;
