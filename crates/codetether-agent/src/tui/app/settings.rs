//! Settings-panel actions: toggles, cycles, and persistence.

use crate::session::Session;
use crate::tui::app::state::App;

include!("settings_modules.rs");
pub use codetether_settings_policy::autocomplete_status_message;

pub async fn set_slash_autocomplete(app: &mut App, session: &mut Session, next: bool) {
    crate::tui::app::settings::host::set(
        app,
        session,
        crate::tui::app::settings::host::Preference::Autocomplete,
        next,
    )
    .await;
}

pub async fn toggle_slash_autocomplete(app: &mut App, session: &mut Session) {
    set_slash_autocomplete(app, session, !app.state.preferences.slash_autocomplete).await;
}
pub use codetether_settings_policy::worktree_status_message;

pub async fn set_use_worktree(app: &mut App, session: &mut Session, next: bool) {
    crate::tui::app::settings::host::set(
        app,
        session,
        crate::tui::app::settings::host::Preference::Worktree,
        next,
    )
    .await;
}
