//! Smart-switch retry scheduling for error events.

#[path = "smart_switch/overload.rs"]
mod overload;

use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::smart_switch::maybe_schedule_smart_switch_retry;
use crate::tui::app::state::App;

pub(super) fn schedule(app: &mut App, slot: &SessionSlot, err: &str) {
    if app
        .state
        .pending_smart_switch_retry
        .as_ref()
        .is_some_and(|pending| pending.restore_from.is_some())
    {
        app.state.status = "Fallback failed; retaining the Codex restore timer".into();
        return;
    }
    let current_model = slot.view().model.as_deref();
    let current_provider = current_model.and_then(|m| m.split('/').next());
    let prompt = app.state.main_inflight_prompt.clone().unwrap_or_default();
    let pending = maybe_schedule_smart_switch_retry(
        err,
        current_model,
        current_provider,
        &app.state.available_models,
        &prompt,
        app.state.smart_switch_retry_count,
        &app.state.smart_switch_attempted_models,
    );
    if let Some(pending) = pending {
        let pending = overload::prepare(pending, err, current_provider, current_model, app);
        app.state.smart_switch_retry_count += 1;
        overload::remember_attempt(app, current_model, &pending.target_model);
        app.state.status = overload::status(&pending, app.state.smart_switch_retry_count);
        app.state.pending_smart_switch_retry = Some(pending);
        return;
    }
    app.state.smart_switch_retry_count = 0;
    app.state.smart_switch_attempted_models.clear();
    app.state.pending_smart_switch_retry = None;
}
