//! Countdown and restoration for temporary Codex overload failover.

#[path = "cooldown/restore.rs"]
mod restore;
pub(super) use restore::{after_fallback, restore};

use crate::tui::app::smart_switch::PendingSmartSwitchRetry;
use crate::tui::app::state::App;

pub(super) fn ready(app: &mut App, pending: &PendingSmartSwitchRetry) -> bool {
    let Some(at) = pending.not_before else {
        return true;
    };
    let remaining = at.saturating_duration_since(std::time::Instant::now());
    if remaining.is_zero() {
        return true;
    }
    app.state.status = format!(
        "Codex overload cooldown: {}s until retry with {}…",
        remaining.as_secs().max(1),
        pending.target_model
    );
    false
}
