//! Key event dispatcher implementation.

use std::{path::Path, sync::Arc};

use crossterm::event::{KeyEvent, KeyEventKind};

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::{app::state::App, worker_bridge::TuiWorkerBridge};

use super::{handle_ctrl_key, handle_unmodified_key};

pub(crate) async fn handle_event(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
    key: KeyEvent,
) -> anyhow::Result<bool> {
    if key.kind != KeyEventKind::Press {
        return Ok(false);
    }
    if super::goal_prompt_key::handle_goal_prompt_key(app, key) {
        return Ok(false);
    }
    if super::editor_key::handle_editor_key(app, key) {
        return Ok(false);
    }
    if let Some(result) = handle_ctrl_key(app, cwd, runtime, key) {
        return result;
    }
    let out = handle_unmodified_key(app, cwd, slot, registry, worker_bridge, runtime, key).await;
    app.state.last_key_at = Some(std::time::Instant::now());
    out
}
