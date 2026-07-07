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
    app.state.cwd_display = cwd.display().to_string();
    app.state.allow_network = allow_network;
    app.state.peer_endpoint_ready = peer_ready;
    app.state.session_id = Some(session.id.clone());
    app.state.status = "Loading providers and workspace...".to_string();
}
