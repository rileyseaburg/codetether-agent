//! Native bindings for ordered key-event dispatch.
use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::{app::state::App, worker_bridge::TuiWorkerBridge};
use crossterm::event::KeyEvent;
use std::{path::Path, sync::Arc};
mod host;
pub(crate) async fn handle_event(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
    key: KeyEvent,
) -> anyhow::Result<bool> {
    codetether_tui_navigation::events::dispatch(
        &mut host::Native {
            app,
            cwd,
            slot,
            registry,
            worker_bridge,
            runtime,
        },
        key,
    )
    .await
}
