//! Replacement-session lifecycle and view reset ordering.
use crate::{Session, View, persistence, preferences};
use std::path::Path;
/// Create and activate a new conversation with selected inherited preferences.
/// # Arguments
/// * `view` - Interactive state and session-list refresh adapter.
/// * `cwd` - Workspace for the final list refresh.
/// * `session` - Current session replaced after its save succeeds.
/// # Returns
/// Completes after status updates; a failed replacement save still resets the view.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_new_session::Session, V: codetether_new_session::View>(view: &mut V, session: &mut S) {
/// codetether_new_session::create(view, std::path::Path::new("."), session).await;
/// # }
/// ```
pub async fn create<S: Session, V: View>(view: &mut V, cwd: &Path, session: &mut S) {
    let mut new_session = match S::new().await {
        Ok(created) => created,
        Err(err) => {
            view.set_status(format!("Failed to create new session: {err}"));
            return;
        }
    };
    if let Err(error) = session.save().await {
        persistence::fail_save_current(view, error);
        return;
    }
    preferences::copy(view, session, &mut new_session);
    *session = new_session;
    persistence::save_new(view, session).await;
    view.set_session_id(session.id().to_string());
    view.clear_messages();
    view.clear_streaming_text();
    view.clear_processing();
    view.clear_request_timing();
    view.scroll_to_bottom();
    view.show_chat();
    view.refresh_sessions(cwd).await;
}
