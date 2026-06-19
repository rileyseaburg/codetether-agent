//! Unique background-agent name selection for `/detach`.

use crate::tui::app::state::App;

/// Choose a unique agent name from `rest`, defaulting to `detached-N`.
pub(super) fn pick_name(app: &App, rest: &str) -> String {
    let base = rest.trim();
    if !base.is_empty() && !app.state.spawned_agents.contains_key(base) {
        return base.to_string();
    }
    (1..)
        .map(|n| format!("detached-{n}"))
        .find(|c| !app.state.spawned_agents.contains_key(c))
        .unwrap_or_else(|| "detached".to_string())
}
