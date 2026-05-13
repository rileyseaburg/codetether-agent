//! Smart-switch retry scheduling for error events.

use crate::session::Session;
use crate::tui::app::smart_switch::{maybe_schedule_smart_switch_retry, smart_switch_max_retries};
use crate::tui::app::state::App;

pub(super) fn schedule(app: &mut App, session: &Session, err: &str) {
    let current_model = session.metadata.model.as_deref();
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
        app.state.smart_switch_retry_count += 1;
        app.state
            .smart_switch_attempted_models
            .push(current_model.unwrap_or("unknown").to_string());
        app.state
            .smart_switch_attempted_models
            .push(pending.target_model.clone());
        app.state.status = format!(
            "Smart switch retry {}/{} → {}",
            app.state.smart_switch_retry_count,
            smart_switch_max_retries(),
            pending.target_model,
        );
        app.state.pending_smart_switch_retry = Some(pending);
        return;
    }
    app.state.smart_switch_retry_count = 0;
    app.state.smart_switch_attempted_models.clear();
    app.state.pending_smart_switch_retry = None;
}
