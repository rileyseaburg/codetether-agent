//! Settings-panel actions: toggles, cycles, and persistence.

use crate::session::Session;
use crate::tui::app::commands::set_auto_apply_edits;
use crate::tui::app::state::App;

#[path = "settings_access_mode.rs"]
pub mod access_mode;
#[path = "settings_bedrock.rs"]
pub mod bedrock;
#[path = "settings_network.rs"]
pub mod network;

pub use bedrock::{bedrock_service_tier_label, cycle_bedrock_service_tier};
pub use network::{network_access_status_message, set_network_access, toggle_network_access};

fn on_off_label(enabled: bool) -> &'static str {
    if enabled { "ON" } else { "OFF" }
}

async fn persist(app: &mut App, session: &mut Session, message: String) {
    match session.save().await {
        Ok(()) => app.state.status = message,
        Err(error) => app.state.status = format!("{message} (not persisted: {error})"),
    }
}

pub fn autocomplete_status_message(enabled: bool) -> String {
    format!("TUI slash autocomplete: {}", on_off_label(enabled))
}

pub async fn set_slash_autocomplete(app: &mut App, session: &mut Session, next: bool) {
    app.state.slash_autocomplete = next;
    session.metadata.slash_autocomplete = next;
    persist(app, session, autocomplete_status_message(next)).await;
}

pub async fn toggle_slash_autocomplete(app: &mut App, session: &mut Session) {
    set_slash_autocomplete(app, session, !app.state.slash_autocomplete).await;
}

pub fn worktree_status_message(enabled: bool) -> String {
    format!("TUI worktree isolation: {}", on_off_label(enabled))
}

pub async fn set_use_worktree(app: &mut App, session: &mut Session, next: bool) {
    app.state.use_worktree = next;
    session.metadata.use_worktree = next;
    persist(app, session, worktree_status_message(next)).await;
}

pub async fn toggle_selected_setting(app: &mut App, session: &mut Session) {
    match app.state.selected_settings_index {
        0 => set_auto_apply_edits(app, session, !app.state.auto_apply_edits).await,
        1 => set_network_access(app, session, !app.state.allow_network).await,
        2 => set_slash_autocomplete(app, session, !app.state.slash_autocomplete).await,
        3 => set_use_worktree(app, session, !app.state.use_worktree).await,
        4 => access_mode::cycle_access_mode(app, session).await,
        5 => cycle_bedrock_service_tier(app, session).await,
        _ => {}
    }
}
