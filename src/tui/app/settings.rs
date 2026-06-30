use crate::session::Session;
use crate::tui::app::commands::set_auto_apply_edits;
use crate::tui::app::state::App;

#[path = "settings_access_mode.rs"]
pub mod access_mode;

fn on_off_label(enabled: bool) -> &'static str {
    if enabled { "ON" } else { "OFF" }
}

async fn persist(app: &mut App, session: &mut Session, message: String) {
    match session.save().await {
        Ok(()) => app.state.status = message,
        Err(error) => app.state.status = format!("{message} (not persisted: {error})"),
    }
}

pub fn network_access_status_message(enabled: bool) -> String {
    format!("TUI network access: {}", on_off_label(enabled))
}

pub async fn set_network_access(app: &mut App, session: &mut Session, next: bool) {
    app.state.allow_network = next;
    session.metadata.allow_network = next;
    if next {
        unsafe { std::env::set_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK", "1") }
    } else {
        unsafe { std::env::remove_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK") }
    }
    persist(app, session, network_access_status_message(next)).await;
}

pub async fn toggle_network_access(app: &mut App, session: &mut Session) {
    set_network_access(app, session, !app.state.allow_network).await;
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
        _ => {}
    }
}
