//! Submit the one-shot TUI prompt queued while a turn was running.

use std::{path::Path, sync::Arc};

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::{App, prompt_queue};
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(super) async fn drain(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) {
    if app.state.processing || !prompt_queue::has_pending() {
        return;
    }
    let Some(prompt) = prompt_queue::take() else {
        return;
    };
    app.state.input = prompt;
    app.state.input_cursor = app.state.input.chars().count();
    app.state.status = "Running queued prompt".to_string();
    crate::tui::app::input::handle_enter(app, cwd, slot, registry, worker_bridge, runtime).await;
}
