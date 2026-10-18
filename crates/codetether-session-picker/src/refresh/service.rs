//! Refresh ordering and empty/error feedback for workspace sessions.
use super::{Source, View};
use std::path::Path;
/// Refresh the picker using the application's combined session listing.
/// # Arguments
/// * `view` - Picker state and status output.
/// * `cwd` - Workspace whose sessions should be listed.
/// # Returns
/// Completes after replacing the list or clearing it after a listing failure.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_picker::refresh::Source, V: codetether_session_picker::refresh::View>(view: &mut V) {
/// codetether_session_picker::refresh::refresh::<S, V>(view, std::path::Path::new(".")).await;
/// # }
/// ```
pub async fn refresh<S: Source, V: View>(view: &mut V, cwd: &Path) {
    tracing::info!(cwd = %cwd.display(), "refresh_sessions: starting");
    match S::list(cwd).await {
        Ok(sessions) => {
            tracing::info!(count = sessions.len(), "refresh_sessions: loaded sessions");
            view.picker().replace_sessions(sessions);
            if view.picker().sessions.is_empty() {
                view.set_status("No sessions found for this workspace".to_string());
            }
        }
        Err(err) => {
            tracing::error!(error = %err, "refresh_sessions: failed");
            view.set_status(format!("Failed to list sessions: {err}"));
            view.picker().clear_sessions();
        }
    }
}
