//! Key event dispatcher implementation.

use std::{path::Path, sync::Arc};

use crossterm::event::KeyEvent;

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
    if !super::key_repeat::dispatchable(key) {
        return Ok(false);
    }
    if let Some(quit) = super::interrupt_key::handle(app, runtime, key) {
        return Ok(quit);
    }
    if super::interlude_key::handle(app, key) {
        return Ok(false);
    }
    if super::goal_prompt_key::handle_goal_prompt_key(app, key) {
        return Ok(false);
    }
    if app.state.spawn_form.is_some()
        && crate::tui::app::spawn_form::handle_spawn_form_key(app, cwd, slot, key).await
    {
        return Ok(false);
    }
    if super::fuzzy_find_key::handle_fuzzy_find_key(app, cwd, key) {
        return Ok(false);
    }
    if super::editor_lsp_key::handle_editor_lsp_key(app, cwd, key).await {
        return Ok(false);
    }
    if super::editor_key::handle_editor_key(app, cwd, key) {
        return Ok(false);
    }
    if let Some(result) = handle_ctrl_key(app, cwd, runtime, key) {
        return result;
    }
    let out = handle_unmodified_key(app, cwd, slot, registry, worker_bridge, runtime, key).await;
    app.state.last_key_at = Some(std::time::Instant::now());
    out
}
