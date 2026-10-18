//! Native adapter for shared keycode dispatch.
use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;
use crossterm::event::KeyEvent;
use std::{path::Path, sync::Arc};
mod host;
pub(super) async fn handle_unmodified_key(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
    key: KeyEvent,
) -> anyhow::Result<bool> {
    Ok(codetether_tui_navigation::keys::dispatch(
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
    .await)
}
