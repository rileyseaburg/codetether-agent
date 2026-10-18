//! Load a selected session, prepare continuation and restore the UI.
use super::{Host, Session, View};
use crate::continuation::Session as _;
use std::path::Path;

/// Load and select a session, reporting failures through the supplied view.
///
/// # Arguments
/// * `app` — UI receiving preferences, transcript and status updates.
/// * `cwd` — Workspace directory used to refresh the session picker.
/// * `session` — Current session, replaced after a successful load.
/// * `session_id` — Native or imported session identifier to load.
///
/// # Returns
/// Updates the session and view; load and continuation errors are shown in status.
///
/// # Examples
/// ```no_run
/// # async fn select<H: codetether_session_resume::interactive::Host>(
/// #     view: &mut impl codetether_session_resume::interactive::View<H::Session>,
/// #     session: &mut H::Session) {
/// codetether_session_resume::interactive::load_selected_session::<H>(
///     view, std::path::Path::new("."), session, "session-id",
/// ).await;
/// # }
/// ```
pub async fn load_selected_session<H: Host>(
    app: &mut impl View<H::Session>,
    cwd: &Path,
    session: &mut H::Session,
    session_id: &str,
) {
    match H::load(session_id).await {
        Ok(loaded) => {
            let dropped = loaded.dropped;
            let file_bytes = loaded.file_bytes;
            *session = loaded.session;
            let original_id = match crate::continuation::fork_if_truncated(session, dropped).await {
                Ok(original) => original,
                Err(error) => {
                    app.set_status(format!("Failed to continue session: {error}"));
                    return;
                }
            };
            session.attach_global_bus_if_missing();
            H::hydrate(session.id()).await;
            app.set_preferences(session.auto_apply_edits(), session.use_worktree());
            app.set_session_id(session.id().to_string());
            let source_id = original_id.as_deref().unwrap_or(session.id()).to_string();
            app.sync_messages(session, &source_id, dropped > 0);
            app.refresh(cwd).await;
            app.clear_filter();
            app.return_to_chat();
            app.set_status(crate::load_status_with_original(
                &session.display_label(),
                session.visible_messages(),
                dropped,
                file_bytes,
                original_id.as_deref(),
            ));
        }
        Err(error) => app.set_status(format!("Failed to load session: {error}")),
    }
}
