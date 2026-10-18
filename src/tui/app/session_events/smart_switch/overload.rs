//! Temporary Codex failover preparation and visible cooldown status.

use crate::tui::app::smart_switch::PendingSmartSwitchRetry;
use crate::tui::app::state::App;
pub(super) fn prepare(
    pending: PendingSmartSwitchRetry,
    error: &str,
    provider: Option<&str>,
    current: Option<&str>,
    app: &App,
) -> PendingSmartSwitchRetry {
    codetether_model_failover::temporary_fallback::prepare(
        pending,
        error,
        provider,
        current,
        &app.state.model_picker.available_models,
    )
}
pub(super) use codetether_model_failover::temporary_fallback::status;

pub(super) fn remember_attempt(app: &mut App, current: Option<&str>, target: &str) {
    app.state.retry_state.remember_attempt(current, target);
}
