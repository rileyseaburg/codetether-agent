//! Restoration after a temporary Codex fallback turn.

use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::smart_switch::PendingSmartSwitchRetry;
use crate::tui::app::state::App;

pub(crate) async fn restore(
    app: &mut App,
    slot: &mut SessionSlot,
    pending: &PendingSmartSwitchRetry,
) -> bool {
    let Some(fallback) = pending.restore_from.as_deref() else {
        return false;
    };
    if app.state.processing {
        app.state.status = format!("Using fallback {fallback}; restore is waiting for completion…");
        app.state.pending_smart_switch_retry = Some(pending.clone());
        return true;
    }
    let Some(session) = slot.borrow() else {
        return true;
    };
    if session.metadata.model.as_deref() != Some(fallback) {
        app.state.status = "Automatic Codex restore skipped after a manual model change".into();
        return true;
    }
    let Some(original) = pending.restore_model.as_deref() else {
        return true;
    };
    if let Some(session) = slot.borrow_mut() {
        session.metadata.model = Some(original.to_string());
        let _ = session.save().await;
    }
    slot.refresh_view();
    app.state.smart_switch_retry_count = 0;
    app.state.smart_switch_attempted_models.clear();
    app.state.status = format!("Codex overload cooldown ended; restored {original}");
    true
}

pub(crate) fn after_fallback(pending: &PendingSmartSwitchRetry) -> Option<PendingSmartSwitchRetry> {
    let original = pending.restore_model.as_ref()?.clone();
    Some(PendingSmartSwitchRetry {
        prompt: pending.prompt.clone(),
        target_model: original.clone(),
        not_before: pending.restore_at,
        restore_from: Some(pending.target_model.clone()),
        restore_at: None,
        restore_model: Some(original),
    })
}
