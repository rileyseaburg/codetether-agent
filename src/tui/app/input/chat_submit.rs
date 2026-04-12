//! Chat prompt submission for the TUI.
//!
//! Validates the input, dispatches slash commands, pushes
//! user messages, and spawns the provider task that runs
//! the prompt in an optional worktree.
//!
//! # Examples
//!
//! ```ignore
//! handle_enter_chat(&mut app, cwd, &mut session, &reg,
//!     &bridge, &tx, &rtx).await;
//! ```

use std::{path::Path, sync::Arc};
use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::{
    commands::handle_slash_command, state::App, worker_bridge::handle_processing_started,
};
use crate::tui::worker_bridge::TuiWorkerBridge;

use super::chat_helpers::{no_provider_error, push_user_messages};
use super::chat_spawn::spawn_provider_task;

/// Submit the chat input to the provider.
///
/// Handles slash commands, pushes user/image messages,
/// starts processing indicators, and spawns the provider
/// task.  If no provider is configured, an error message
/// is shown instead.
///
/// # Examples
///
/// ```ignore
/// handle_enter_chat(&mut app, cwd, &mut session, &reg,
///     &bridge, &tx, &rtx).await;
/// ```
pub(super) async fn handle_enter_chat(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
) {
    if app.state.processing {
        app.state.status = "Still processing previous request…".to_string();
        return;
    }

    let prompt = app.state.input.trim().to_string();
    if !prompt.is_empty() {
        app.state.push_history(prompt.clone());
    }
    if prompt.starts_with('/') {
        handle_slash_command(app, cwd, session, registry.as_ref(), &prompt).await;
        app.state.clear_input();
        return;
    }

    let pending_images = std::mem::take(&mut app.state.pending_images);
    if prompt.is_empty() && pending_images.is_empty() {
        return;
    }

    push_user_messages(app, &prompt, &pending_images);
    let effective_prompt = if let Some(prefix) = app.state.steering_prompt_prefix() {
        format!("{prefix}\nUser request:\n{prompt}")
    } else {
        prompt.clone()
    };
    let steering_count = app.state.steering_count();
    app.state.clear_input();
    app.state.clear_steering();
    handle_processing_started(app, worker_bridge).await;
    app.state.begin_request_timing();
    app.state.main_inflight_prompt = Some(prompt.clone());
    app.state.status = if steering_count > 0 {
        format!("Submitting prompt with {steering_count} steering item(s)…")
    } else {
        "Submitting prompt…".to_string()
    };
    app.state.scroll_to_bottom();

    if let Some(registry) = registry {
        spawn_provider_task(
            app,
            cwd,
            session,
            registry,
            &effective_prompt,
            pending_images,
            event_tx,
            result_tx,
        )
        .await;
    } else {
        no_provider_error(app, worker_bridge).await;
    }
}
