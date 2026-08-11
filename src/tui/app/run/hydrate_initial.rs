//! Initial (pre-startup) TUI app hydration.

use std::path::Path;

use crate::session::Session;
use crate::tui::app::state::App;

/// Hydrates `app` with the fields known before the slow startup phase completes.
pub(super) fn initial(
    app: &mut App,
    cwd: &Path,
    allow_network: bool,
    peer_ready: bool,
    session: &Session,
) {
    loading(app, cwd, allow_network);
    app.state.peer_endpoint_ready = peer_ready;
    app.state.session_id = Some(session.id.clone());
}

/// Prepares the application state shown while startup dependencies load.
///
/// # Arguments
///
/// * `app` — State updated for the loading frame.
/// * `cwd` — Workspace path displayed in the frame.
/// * `allow_network` — Requested network policy shown during startup.
///
/// # Examples
///
/// ```text
/// Loading providers and workspace...
/// ```
pub(super) fn loading(app: &mut App, cwd: &Path, allow_network: bool) {
    app.state.cwd_display = cwd.display().to_string();
    app.state.allow_network = allow_network;
    app.state.status = "Loading providers and workspace...".to_string();
}

#[cfg(test)]
#[path = "hydrate_initial_tests.rs"]
mod tests;
