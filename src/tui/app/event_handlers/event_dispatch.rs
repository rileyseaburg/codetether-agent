//! Key event dispatcher implementation.

use std::{path::Path, sync::Arc};

use crossterm::event::KeyEvent;

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::{app::state::App, worker_bridge::TuiWorkerBridge};

use super::{keybinds::handle_unmodified_key, keyboard::handle_ctrl_key};

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
    if crate::tui::app::input::goal_answer::editor::handle(app, slot, runtime, key).await
        || super::approval_key::scroll(app, key)
    {
        return Ok(false);
    }
    if let Some(resume) = super::answer_review_key::handle(app, slot, key).await? {
        if resume {
            crate::tui::app::input::sessions::goal_autostart::resume(
                app,
                cwd,
                slot,
                registry,
                worker_bridge,
                runtime,
            )
            .await;
        }
        return Ok(false);
    }
    if let Some(quit) = super::event_priority::handle(app, cwd, slot, runtime, key).await {
        return Ok(quit);
    }
    if let Some(result) = handle_ctrl_key(app, cwd, runtime, key) {
        return result;
    }
    let out = handle_unmodified_key(app, cwd, slot, registry, worker_bridge, runtime, key).await;
    app.state.last_key_at = Some(std::time::Instant::now());
    out
}
