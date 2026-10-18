//! Shared save-result formatting for settings updates.
use super::{Session, View};
/// Persist current session settings and display the result.
/// # Arguments
/// * `view` - Interactive status output.
/// * `session` - Session storage adapter.
/// * `message` - Status text for a successful save.
/// # Returns
/// Completes after reporting success or appending the persistence failure.
/// # Examples
/// ```no_run
/// # async fn example<V: codetether_settings_policy::persistence::View, S: codetether_settings_policy::persistence::Session>(view: &mut V, session: &S) {
/// codetether_settings_policy::persistence::save(view, session, "Updated".into()).await;
/// # }
/// ```
pub async fn save<V: View, S: Session>(view: &mut V, session: &S, message: String) {
    match session.save().await {
        Ok(()) => view.set_status(message),
        Err(error) => view.set_status(format!("{message} (not persisted: {error})")),
    }
}
