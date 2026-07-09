//! Chat prompt submission for the TUI.
//!
//! Validates the input, dispatches `!shell` and slash commands, pushes
//! user messages, and delegates to
//! [`super::chat_submit_dispatch::dispatch_prompt`].
//!
//! Mid-stream behavior: while a turn is in flight, plain-text Enter stores
//! one follow-up prompt and the event loop submits it after completion.

use std::{path::Path, sync::Arc};

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

#[cfg(test)]
#[path = "chat_submit_queue_tests.rs"]
mod queue_tests;

/// Submit the chat input to the provider.
///
/// Handles `!shell` and slash commands, pushes user/image messages,
/// then delegates prompt preparation and dispatch. While a previous
/// request is still in flight, plain-text input is queued as the next
/// prompt.
pub(super) async fn handle_enter_chat(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) {
    super::image_sidecar_recover::recover_pasted_images(app);
    let prompt = app.state.input.trim().to_string();
    if !prompt.is_empty() {
        app.state.push_history(prompt.clone());
    }
    let prompt =
        super::mention_route::route_prompt(&prompt, app.state.active_spawned_agent.as_deref());
    if super::forage_offer::intercept(app, slot, &prompt) {
        return;
    }
    if super::continue_command::run(app, cwd, slot, registry, worker_bridge, runtime, &prompt).await
    {
        return;
    }
    if super::chat_submit_slash::run(app, cwd, slot, registry, &prompt).await {
        return;
    }
    if app.state.processing {
        crate::tui::app::state::prompt_queue::store(app, prompt);
        return;
    }
    super::chat_submit_finish::finish_submit(
        app,
        cwd,
        slot,
        registry,
        worker_bridge,
        runtime,
        &prompt,
    )
    .await;
}
