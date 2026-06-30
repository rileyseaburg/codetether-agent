//! Access-mode cycling for the Settings panel.
//!
//! Exposes access mode (`ask` → `approve` → `full`) as a selectable,
//! keypress-cycled Settings row rather than a typed slash command.

use crate::config::{AccessMode, Config};
use crate::session::Session;
use crate::tui::app::state::App;

/// Return the access mode the Settings panel currently reflects.
pub fn current_access_mode() -> AccessMode {
    crate::tui::ui::trust_status::current_status()
        .and_then(|status| status.access_mode)
        .unwrap_or(AccessMode::Ask)
}

fn next_access_mode(mode: AccessMode) -> AccessMode {
    match mode {
        AccessMode::Ask => AccessMode::Approve,
        AccessMode::Approve => AccessMode::Full,
        AccessMode::Full => AccessMode::Ask,
    }
}

/// Human-readable label for an access mode.
pub fn label(mode: AccessMode) -> &'static str {
    match mode {
        AccessMode::Ask => "ask",
        AccessMode::Approve => "approve",
        AccessMode::Full => "full",
    }
}

/// Cycle to the next access mode and apply it to the live session.
pub async fn cycle_access_mode(app: &mut App, session: &mut Session) {
    let next = next_access_mode(current_access_mode());
    Config::apply_process_access_mode_override(Some(next));
    let cwd = std::env::current_dir().unwrap_or_default();
    if let Ok(cfg) = Config::load_for_workspace(&cwd).await {
        crate::tui::ui::trust_status::set_from_config(&cfg);
        session.apply_config(&cfg, None);
    }
    app.state.status = format!("Access mode: {}", label(next));
}
