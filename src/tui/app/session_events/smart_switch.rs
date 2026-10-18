//! Smart-switch retry scheduling for error events.

#[path = "smart_switch/overload.rs"]
mod overload;

use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;

pub(super) fn schedule(app: &mut App, slot: &SessionSlot, err: &str) {
    if app
        .state
        .retry_state
        .pending
        .as_ref()
        .is_some_and(|pending| pending.restore_from.is_some())
    {
        app.state.status = "Fallback failed; retaining the Codex restore timer".into();
        return;
    }
    let current_model = slot.view().model.as_deref();
    let current_provider = current_model.and_then(|m| m.split('/').next());
    let prompt = app.state.main_inflight_prompt.clone().unwrap_or_default();
    let pending = app.state.retry_state.select(
        err,
        current_model,
        current_provider,
        &app.state.model_picker.available_models,
        &prompt,
    );
    if let Some(pending) = pending {
        let pending = overload::prepare(pending, err, current_provider, current_model, app);
        app.state.retry_state.count += 1;
        overload::remember_attempt(app, current_model, &pending.target_model);
        app.state.status = overload::status(&pending, app.state.retry_state.count);
        app.state.retry_state.pending = Some(pending);
        return;
    }
    app.state.retry_state.clear();
}
