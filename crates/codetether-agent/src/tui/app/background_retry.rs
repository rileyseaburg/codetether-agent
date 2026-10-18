//! Smart-switch scheduling for runtime failures.
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;
pub(super) fn schedule(app: &mut App, slot: &SessionSlot, error: &str) {
    let model = slot.view().model.as_deref();
    let prompt = app
        .state
        .watchdog
        .inflight_prompt
        .clone()
        .unwrap_or_default();
    app.state.retry_state.schedule_runtime(
        error,
        model,
        &app.state.model_picker.available_models,
        &prompt,
    );
}
