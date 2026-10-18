//! New-session storage failure reporting and bus attachment.
use crate::{Session, View};
pub(super) fn fail_save_current<V: View>(view: &mut V, error: anyhow::Error) {
    tracing::warn!(error = %error, "Failed to save current session before /new");
    view.set_status(format!(
        "Failed to save current session before creating new session: {error}"
    ));
}
pub(super) async fn save_new<S: Session, V: View>(view: &mut V, session: &mut S) {
    session.attach_global_bus_if_missing();
    if let Err(error) = session.save().await {
        tracing::warn!(error = %error, "Failed to save new session");
        view.set_status(format!(
            "New chat session created, but failed to persist: {error}"
        ));
    } else {
        view.set_status("New chat session".to_string());
    }
}
