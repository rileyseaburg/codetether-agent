use std::{path::Path, sync::Arc};

#[path = "inbox/live.rs"]
pub(crate) mod live;

use crate::provider::ProviderRegistry;
use crate::tui::app::input::chat_submit_dispatch::dispatch_prompt;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::{IncomingTask, TuiWorkerBridge};

mod host;
mod presentation;
pub(crate) async fn trigger_next(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) {
    codetether_a2a_interactive::trigger_next(&mut host::Native {
        app,
        cwd,
        slot,
        registry,
        worker_bridge,
        runtime,
    })
    .await;
}
