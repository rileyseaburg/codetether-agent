//! Access-mode cycling for the Settings panel.
//!
//! Exposes access mode (`ask` → `approve` → `full`) as a selectable,
//! keypress-cycled Settings row rather than a typed slash command.

use crate::config::AccessMode;
use crate::session::Session;
use crate::tui::app::state::App;

/// Return the access mode the Settings panel currently reflects.
pub fn current_access_mode() -> AccessMode {
    crate::tui::ui::trust_status::current_status()
        .and_then(|status| status.access_mode)
        .unwrap_or(AccessMode::Ask)
}

pub use codetether_settings_policy::access_label as label;

/// Cycle to the next access mode and apply it to the live session.
pub async fn cycle_access_mode(app: &mut App, session: &mut Session) {
    super::host::cycle_access(app, session).await;
}
