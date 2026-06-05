use std::path::Path;

use crate::session::Session;
use crate::tui::app::message_text::sync_messages_from_session;
use crate::tui::app::state::App;
use crate::tui::models::WorkspaceSnapshot;
use crate::tui::worker_bridge::TuiWorkerBridge;

use super::session_outcome::SessionLoadOutcome;

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

pub(super) fn complete(
    app: &mut App,
    allow_network: bool,
    session: &mut Session,
    worker_bridge: Option<&TuiWorkerBridge>,
    outcome: SessionLoadOutcome,
    workspace: WorkspaceSnapshot,
) {
    app.state.workspace = workspace;
    app.state.auto_apply_edits = session.metadata.auto_apply_edits;
    app.state.allow_network = session.metadata.allow_network || allow_network;
    app.state.slash_autocomplete = session.metadata.slash_autocomplete;
    app.state.use_worktree = session.metadata.use_worktree;
    app.state.session_id = Some(session.id.clone());
    session.metadata.allow_network = app.state.allow_network;
    sync_messages_from_session(app, session);
    super::worker_attach::attach(app, worker_bridge);
    app.state.refresh_slash_suggestions();
    app.state.move_cursor_end();
    app.state.status = outcome.status(&session.id);
}
