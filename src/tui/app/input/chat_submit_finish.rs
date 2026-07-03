//! Final stage of chat submission: take pending attachments, expand paste
//! placeholders, push the user message, and dispatch to the provider.

use std::{path::Path, sync::Arc};

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

use super::chat_helpers::push_user_messages;
use super::chat_submit_dispatch::dispatch_prompt;
use super::pasted_text::expand_paste_placeholders;

/// Dispatch `prompt` plus any pending images/text-paste sidecars.
pub(super) async fn finish_submit(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
    prompt: &str,
) {
    let pending_images = std::mem::take(&mut app.state.pending_images);
    let pending_text_pastes = std::mem::take(&mut app.state.pending_text_pastes);
    if prompt.is_empty() && pending_images.is_empty() {
        return;
    }
    // Chat history shows the compact placeholder; the agent receives the
    // expanded full paste content wrapped in delimiters.
    let agent_prompt = expand_paste_placeholders(prompt, &pending_text_pastes);
    push_user_messages(app, prompt, &pending_images);
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
