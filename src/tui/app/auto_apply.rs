//! Auto-apply edit toggle helpers for the TUI.
//!
//! Encapsulates the "auto-apply edits" preference: status strings plus the
//! setter that persists the flag onto the session metadata.

use crate::session::Session;
use crate::tui::app::state::App;
pub use codetether_settings_policy::auto_apply_status_message;

/// Set the auto-apply-edits flag and persist it onto the session.
pub async fn set_auto_apply_edits(app: &mut App, session: &mut Session, next: bool) {
    crate::tui::app::settings::host::set(
        app,
        session,
        crate::tui::app::settings::host::Preference::AutoApply,
        next,
    )
    .await;
}

/// Flip the current auto-apply-edits flag.
pub async fn toggle_auto_apply_edits(app: &mut App, session: &mut Session) {
    set_auto_apply_edits(app, session, !app.state.auto_apply_edits).await;
}
