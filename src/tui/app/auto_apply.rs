//! Auto-apply edit toggle helpers for the TUI.
//!
//! Encapsulates the "auto-apply edits" preference: status strings plus the
//! setter that persists the flag onto the session metadata.

use crate::session::Session;
use crate::tui::app::state::App;

fn auto_apply_flag_label(enabled: bool) -> &'static str {
    if enabled { "ON" } else { "OFF" }
}

/// Human-readable status line describing the auto-apply edit state.
pub fn auto_apply_status_message(enabled: bool) -> String {
    format!("TUI edit auto-apply: {}", auto_apply_flag_label(enabled))
}

/// Set the auto-apply-edits flag and persist it onto the session.
pub async fn set_auto_apply_edits(app: &mut App, session: &mut Session, next: bool) {
    app.state.auto_apply_edits = next;
    session.metadata.auto_apply_edits = next;
    app.state.status = match session.save().await {
        Ok(()) => auto_apply_status_message(next),
        Err(error) => format!(
            "{} (not persisted: {error})",
            auto_apply_status_message(next)
        ),
    };
}

/// Flip the current auto-apply-edits flag.
pub async fn toggle_auto_apply_edits(app: &mut App, session: &mut Session) {
    set_auto_apply_edits(app, session, !app.state.auto_apply_edits).await;
}
