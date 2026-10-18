//! Settings-panel actions: toggles, cycles, and persistence.

use crate::session::Session;
use crate::tui::app::state::App;

include!("settings_modules.rs");

async fn persist(app: &mut App, session: &mut Session, message: String) {
    match session.save().await {
        Ok(()) => app.state.status = message,
        Err(error) => app.state.status = format!("{message} (not persisted: {error})"),
    }
}
pub use codetether_settings_policy::autocomplete_status_message;

pub async fn set_slash_autocomplete(app: &mut App, session: &mut Session, next: bool) {
    app.state.slash_autocomplete = next;
    session.metadata.slash_autocomplete = next;
    persist(app, session, autocomplete_status_message(next)).await;
}

pub async fn toggle_slash_autocomplete(app: &mut App, session: &mut Session) {
    set_slash_autocomplete(app, session, !app.state.slash_autocomplete).await;
}
pub use codetether_settings_policy::worktree_status_message;

pub async fn set_use_worktree(app: &mut App, session: &mut Session, next: bool) {
    app.state.use_worktree = next;
    session.metadata.use_worktree = next;
    persist(app, session, worktree_status_message(next)).await;
}
