//! Temporary Codex failover preparation and visible cooldown status.

use std::time::Instant;

use crate::tui::app::smart_switch::{
    PendingSmartSwitchRetry, codex_overload, codex_overload_cooldown, codex_overload_fallback,
};
use crate::tui::app::state::App;

pub(super) fn prepare(
    mut pending: PendingSmartSwitchRetry,
    error: &str,
    provider: Option<&str>,
    current: Option<&str>,
    app: &App,
) -> PendingSmartSwitchRetry {
    if codex_overload(error) && provider == Some("openai-codex") {
        if let Some(fallback) = codex_overload_fallback(current, &app.state.available_models) {
            pending.target_model = fallback;
        }
        pending.restore_at = Some(Instant::now() + codex_overload_cooldown(error));
        pending.not_before = None;
        pending.restore_from = Some(pending.target_model.clone());
        pending.restore_model = current.map(ToString::to_string);
    }
    pending
}

pub(super) fn status(pending: &PendingSmartSwitchRetry, count: u32) -> String {
    let Some(restore) = pending.restore_model.as_ref() else {
        return format!(
            "Smart switch retry {}/{} → {}",
            count,
            crate::tui::app::smart_switch::smart_switch_max_retries(),
            pending.target_model
        );
    };
    let seconds = pending
        .restore_at
        .map(|at| at.saturating_duration_since(Instant::now()).as_secs())
        .unwrap_or_default();
    format!(
        "Codex overloaded; retrying with {} in {}s; restore {} afterward…",
        pending.target_model, seconds, restore
    )
}

pub(super) fn remember_attempt(app: &mut App, current: Option<&str>, target: &str) {
    app.state
        .smart_switch_attempted_models
        .push(current.unwrap_or("unknown").to_string());
    app.state
        .smart_switch_attempted_models
        .push(target.to_string());
}
