//! Child-only focus cycling while the transcript detail pane is open.

use crate::tui::app::state::App;

pub(super) fn cycle(app: &mut App) {
    let names = super::agent_names(app);
    if names.is_empty() {
        app.state.active_spawned_agent = None;
        return;
    }
    let next = app
        .state
        .active_spawned_agent
        .as_ref()
        .and_then(|current| names.iter().position(|name| name == current))
        .map_or(0, |index| (index + 1) % names.len());
    super::set_focus(app, Some(names[next].clone()));
    app.state.subagent_detail_scroll = 0;
}

#[cfg(test)]
#[path = "agent_focus_detail_tests.rs"]
mod tests;
