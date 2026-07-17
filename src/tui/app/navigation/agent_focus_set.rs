//! Shared state transition for agent-rail focus changes.

use crate::tui::app::state::App;

pub(super) fn set_focus(app: &mut App, next: Option<String>) {
    app.state.status = next.as_ref().map_or_else(
        || "Focused main chat".to_string(),
        |name| format!("Focused agent: {name}"),
    );
    super::swarm::sync(app, next.as_deref());
    app.state.active_spawned_agent = next;
    crate::tui::app::message_cache_invalidate::clear(&mut app.state);
    app.state.scroll_to_bottom();
}
