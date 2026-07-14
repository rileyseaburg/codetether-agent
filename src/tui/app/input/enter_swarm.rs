//! Enter-key handling for the swarm view.
//!
//! A running swarm is detached and must not lock the chat session. When the
//! input box has text we route Enter to the chat submit handler so the user
//! can keep talking to the agent; an empty box opens the agent detail pane.

use std::{path::Path, sync::Arc};

use crate::provider::ProviderRegistry;
use crate::tui::app::input::chat_submit::handle_enter_chat;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;
use crate::tui::worker_bridge::TuiWorkerBridge;

/// Route Enter while in the swarm view (submit chat vs. open detail).
pub(super) async fn dispatch_swarm_enter(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) {
    if app.state.input.trim().is_empty() {
        app.state.swarm.enter_detail();
    } else {
        handle_enter_chat(app, cwd, slot, registry, worker_bridge, runtime).await;
    }
}

/// Open the selected swarm worker from the unified agents dashboard.
pub(super) fn dispatch_subagents_enter(app: &mut App) {
    if app.state.swarm.subtasks.is_empty() {
        app.state.status = "No swarm agents to inspect".to_string();
        return;
    }
    app.state.set_view_mode(ViewMode::Swarm);
    app.state.swarm.enter_detail();
    app.state.status = "Swarm agent detail".to_string();
}

#[cfg(test)]
#[path = "enter_swarm_tests.rs"]
mod tests;
