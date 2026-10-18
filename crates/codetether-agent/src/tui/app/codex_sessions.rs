//! Native bindings for interactive session loading and archive imports.
mod host;
use crate::{session::Session, tui::app::state::App};
use std::path::Path;
/// Load a selected session through shared continuation and presentation policy.
pub async fn load_selected_session(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    session_id: &str,
) {
    codetether_session_resume::interactive::load_selected_session::<host::Native>(
        app, cwd, session, session_id,
    )
    .await;
}
/// Import workspace sessions and refresh the picker after success.
pub async fn import_workspace_sessions(app: &mut App, cwd: &Path) {
    codetether_session_resume::interactive::import_workspace_sessions::<host::Native>(app, cwd)
        .await;
}
