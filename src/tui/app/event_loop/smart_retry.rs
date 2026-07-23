//! Smart-switch retry execution after a provider failure.
//!
//! When a prompt fails on one model the TUI can schedule a
//! retry on a fallback model.  This module executes that
//! pending retry after the failed result is applied.
//!
//! # Examples
//!
//! ```ignore
//! execute_smart_switch_retry(
//!     &mut app, &mut slot, &registry, &runtime,
//! ).await;
//! ```

#[path = "smart_retry/cooldown.rs"]
mod cooldown;

use std::sync::Arc;
use std::time::Instant;

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::smart_switch::should_execute_smart_switch;
use crate::tui::app::state::App;

/// Execute a pending smart-switch retry if scheduled.
///
/// Checks whether a retry was queued by the result handler
/// and, if valid, switches the model and spawns a new
/// prompt task.
///
/// # Examples
///
/// ```ignore
/// execute_smart_switch_retry(
///     &mut app, &mut slot, &registry, &runtime,
/// ).await;
/// ```
pub(super) async fn execute_smart_switch_retry(
    app: &mut App,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    runtime: &TuiSessionHandle,
) {
    let Some(pending) = app.state.pending_smart_switch_retry.take() else {
        return;
    };
    if !cooldown::ready(app, &pending) {
        app.state.pending_smart_switch_retry = Some(pending);
        return;
    }
    if cooldown::restore(app, slot, &pending).await {
        return;
    }
    let Some(session) = slot.borrow() else { return };
    let current_model = session.metadata.model.as_deref();
    if !should_execute_smart_switch(current_model, Some(&pending)) {
        return;
    }

    if let Some(session) = slot.borrow_mut() {
        session.metadata.model = Some(pending.target_model.clone());
        let _ = session.save().await;
    }

    app.state.processing = true;
    app.state.begin_request_timing();
    app.state.main_inflight_prompt = Some(pending.prompt.clone());
    app.state.main_last_event_at = Some(Instant::now());
    app.state.status = format!("Retrying with {}…", pending.target_model);

    if let Some(registry) = registry.as_ref() {
        super::smart_retry_submit::submit(slot, &pending.prompt, runtime, registry).await;
    }
    if let Some(restore) = cooldown::after_fallback(&pending) {
        app.state.pending_smart_switch_retry = Some(restore);
    }
}
