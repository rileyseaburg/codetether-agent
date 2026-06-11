//! Chat prompt submission for the TUI.
//!
//! Validates the input, dispatches slash commands, pushes
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

use super::chat_helpers::push_user_messages;
use super::chat_submit_dispatch::dispatch_prompt;
use super::pasted_text::expand_paste_placeholders;

#[cfg(test)]
#[path = "chat_submit_queue_tests.rs"]
mod queue_tests;

/// Submit the chat input to the provider.
///
/// Handles slash commands, pushes user/image messages, then delegates
/// prompt preparation and dispatch. While a previous request is still
/// in flight, plain-text input is queued as the next prompt.
pub(super) async fn handle_enter_chat(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) {
    let prompt = app.state.input.trim().to_string();
    if !prompt.is_empty() {
        app.state.push_history(prompt.clone());
    }
    let prompt = if let Some((agent_name, msg)) = super::mention_route::parse_mention(&prompt) {
        tracing::info!(agent = %agent_name, "Mention detected, routing as prefixed message");
        format!("[to @{}] {}", agent_name, msg)
    } else {
        prompt
    };
    if super::chat_submit_slash::run(app, cwd, slot, registry, &prompt).await {
        return;
    }
    if app.state.processing {
        crate::tui::app::state::prompt_queue::store(app, prompt);
        return;
    }

    let pending_images = std::mem::take(&mut app.state.pending_images);
    let pending_text_pastes = std::mem::take(&mut app.state.pending_text_pastes);
    if prompt.is_empty() && pending_images.is_empty() {
        return;
    }

    // The user-facing chat history shows the compact placeholder so a
    // 1-line summary stands in for what would otherwise be an
    // 1000-line wall of text. The agent receives the expanded form
    // with the full pasted content wrapped in delimiters.
    let agent_prompt = expand_paste_placeholders(&prompt, &pending_text_pastes);

    push_user_messages(app, &prompt, &pending_images);
    dispatch_prompt(
        app,
        cwd,
        slot,
        registry,
        worker_bridge,
        &agent_prompt,
        pending_images,
        runtime,
    )
    .await;
}
