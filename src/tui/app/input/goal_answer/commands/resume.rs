//! Resume updated goals only after cancellation has returned the canonical session.

use crate::provider::ProviderRegistry;
use crate::tui::{
    app::{
        session_runtime::{SessionSlot, TuiSessionHandle},
        state::App,
    },
    worker_bridge::TuiWorkerBridge,
};
use std::{path::Path, sync::Arc};

pub(crate) async fn drain(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) {
    if app.state.processing || slot.borrow().is_none() || registry.is_none() {
        return;
    }

    if super::pending::take(slot.view().id()) {
        crate::tui::app::input::sessions::goal_autostart::resume(
            app, cwd, slot, registry, bridge, runtime,
        )
        .await;
    }
}

/// Schedule fresh goal continuation after the old turn returns ownership.
pub(crate) fn restart(app: &mut App, id: &str, runtime: &TuiSessionHandle) {
    super::pending::mark(id);
    runtime.request_cancel_current();
    app.state.watchdog_notification = None;
    app.state.main_inflight_prompt = None;
}
