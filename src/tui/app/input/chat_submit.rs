//! Chat prompt submission for the TUI.
//!
//! Validates the input, dispatches slash commands, pushes
//! user messages, and delegates to
//! [`super::chat_submit_dispatch::dispatch_prompt`].

use std::{path::Path, sync::Arc};
use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::commands::handle_slash_command;
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

use super::chat_helpers::push_user_messages;
use super::chat_steer_queue::queue_steering_while_processing;
use super::chat_submit_dispatch::dispatch_prompt;

/// Submit the chat input to the provider.
///
/// Handles slash commands, pushes user/image messages,
/// then delegates prompt preparation and dispatch. While a
/// previous request is still in flight, plain-text input is
/// queued as steering instead of being rejected, so users
/// can steer mid-stream.
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
        queue_steering_while_processing(app, &prompt);
        return;
    }

    let pending_images = std::mem::take(&mut app.state.pending_images);
    if prompt.is_empty() && pending_images.is_empty() {
        return;
    }

    push_user_messages(app, &prompt, &pending_images);
    dispatch_prompt(
        app,
        cwd,
        session,
        registry,
        worker_bridge,
        &prompt,
        pending_images,
        event_tx,
        result_tx,
    )
    .await;
}
