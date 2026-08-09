use crate::session::Session;
use crate::tui::app::state::App;

pub(super) fn fail_create(app: &mut App, err: anyhow::Error) {
    app.state.status = format!("Failed to create new session: {err}");
}

pub(super) fn fail_save_current(app: &mut App, error: anyhow::Error) {
    tracing::warn!(error = %error, "Failed to save current session before /new");
    app.state.status =
        format!("Failed to save current session before creating new session: {error}");
}

pub(super) async fn persist_new_session(app: &mut App, session: &mut Session) {
    session.attach_global_bus_if_missing();
    if let Err(error) = session.save().await {
        tracing::warn!(error = %error, "Failed to save new session");
        app.state.status = format!("New chat session created, but failed to persist: {error}");
    } else {
        app.state.status = "New chat session".to_string();
    }
}
