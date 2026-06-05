//! Smart-switch scheduling for runtime failures.

use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;

pub(super) fn schedule(app: &mut App, slot: &SessionSlot, error: &str) {
    let model = slot.view().model.as_deref();
    let provider = model.and_then(|m| m.split('/').next());
    let prompt = app.state.main_inflight_prompt.clone().unwrap_or_default();
    let pending = crate::tui::app::smart_switch::maybe_schedule_smart_switch_retry(
        error,
        model,
        provider,
        &app.state.available_models,
        &prompt,
        app.state.smart_switch_retry_count,
        &app.state.smart_switch_attempted_models,
    );
    if let Some(pending) = pending {
        app.state.smart_switch_retry_count += 1;
        app.state
            .smart_switch_attempted_models
            .push(model.unwrap_or("unknown").into());
        app.state
            .smart_switch_attempted_models
            .push(pending.target_model.clone());
        app.state.pending_smart_switch_retry = Some(pending);
    }
}
