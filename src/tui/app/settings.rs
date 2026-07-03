//! Settings-panel actions: toggles, cycles, and persistence.

use crate::session::Session;
use crate::tui::app::commands::set_auto_apply_edits;
use crate::tui::app::state::App;

#[path = "settings_access_mode.rs"]
pub mod access_mode;
#[path = "settings_bedrock.rs"]
pub mod bedrock;
#[path = "settings_bedrock_effort.rs"]
pub mod bedrock_effort;
#[path = "settings_dispatch.rs"]
pub mod dispatch;
#[path = "settings_network.rs"]
pub mod network;

pub use bedrock::bedrock_service_tier_label;
pub use bedrock::cycle_bedrock_service_tier;
pub use bedrock_effort::{bedrock_thinking_effort_label, cycle_bedrock_thinking_effort};
pub use dispatch::toggle_selected_setting;
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
