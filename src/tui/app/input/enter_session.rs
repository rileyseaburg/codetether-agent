//! Enter-key handlers that require mutable session ownership.

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;
use std::{path::Path, sync::Arc};

pub(super) async fn sessions(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) {
    super::super::sessions::handle_enter_sessions(app, cwd, slot, registry, worker_bridge, runtime)
        .await;
}

pub(super) async fn settings(app: &mut App, slot: &mut SessionSlot) {
    if let Some(session) = slot.borrow_mut() {
        crate::tui::app::settings::toggle_selected_setting(app, session).await;
    }
}
