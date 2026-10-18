//! Submit the one-shot TUI prompt queued while a turn was running.

mod host;
use std::{path::Path, sync::Arc};

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) async fn drain(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) {
    codetether_chat_submit::queued::drain(&mut host::Native {
        app,
        cwd,
        slot,
        registry,
        worker_bridge,
        runtime,
    })
    .await;
}
