use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub async fn trigger_next(
    app: &mut App,
    cwd: &std::path::Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) {
    crate::tui::app::inbox::trigger_next(app, cwd, slot, registry, worker_bridge, runtime).await;
}
