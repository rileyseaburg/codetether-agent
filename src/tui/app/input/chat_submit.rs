//! Chat prompt submission for the TUI.
//!
//! Validates the input, dispatches slash commands, pushes
//! user messages, and delegates to
//! [`super::chat_submit_dispatch::dispatch_prompt`].
//!
//! Mid-stream behavior (Anthropic-style): while a turn is in flight,
//! plain-text Enter is a no-op that shows a hint directing the user to
//! `Ctrl+C` (interrupt) or `/ask` (side question). The typed text stays
//! in the input buffer so it is not lost. Slash commands are always
//! dispatched immediately because they are either local (e.g. `/help`)
//! or non-conflicting with the main turn (e.g. `/ask`).

use std::{path::Path, sync::Arc};
use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::commands::handle_slash_command;
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

use super::chat_helpers::push_user_messages;
use super::chat_submit_dispatch::dispatch_prompt;
use super::pasted_text::expand_paste_placeholders;

/// Submit the chat input to the provider.
///
/// Handles slash commands, pushes user/image messages, then delegates
/// prompt preparation and dispatch. While a previous request is still
/// in flight, plain-text input is **not** auto-queued: users should
/// press `Ctrl+C` to interrupt the current turn (the partial transcript
/// is preserved in history) or `/ask <question>` for a side question.
pub(super) async fn handle_enter_chat(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
) {
    let prompt = app.state.input.trim().to_string();
    if !prompt.is_empty() {
        app.state.push_history(prompt.clone());
    }
    if prompt.starts_with('/') {
        handle_slash_command(app, cwd, session, registry.as_ref(), &prompt).await;
        app.state.clear_input();
        return;
    }
    if app.state.processing {
        // Don't discard the user's typed text; just guide them.
        app.state.status =
            "Agent is still responding — press Ctrl+C to interrupt, or use /ask <question> \
             for a side question. Your text stays in the input."
                .to_string();
        return;
    }

    if prompt.is_empty() && app.state.pending_images.is_empty() {
        return;
    }
    let pending_images = std::mem::take(&mut app.state.pending_images);
    let pending_text_pastes = std::mem::take(&mut app.state.pending_text_pastes);

    // The user-facing chat history shows the compact placeholder so a
    // 1-line summary stands in for what would otherwise be an
    // 1000-line wall of text. The agent receives the expanded form
    // with the full pasted content wrapped in delimiters.
    let agent_prompt = expand_paste_placeholders(&prompt, &pending_text_pastes);

    push_user_messages(app, &prompt, &pending_images);
    dispatch_prompt(
        app,
        cwd,
        session,
        registry,
        worker_bridge,
        &agent_prompt,
        pending_images,
        event_tx,
        result_tx,
    )
    .await;
}
