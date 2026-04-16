//! Prompt preparation and provider dispatch.
//!
//! Handles steering prefix assembly, UI state updates, and
//! delegation to [`super::chat_spawn::spawn_provider_task`].

use std::{path::Path, sync::Arc};
use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{ImageAttachment, Session, SessionEvent};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

use super::chat_helpers::no_provider_error;
use super::chat_spawn::spawn_provider_task;

/// Prepare the effective prompt and dispatch to provider.
///
/// Applies steering prefix, updates UI state, and either
/// spawns the provider task or shows an error.
pub(super) async fn dispatch_prompt(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    prompt: &str,
    pending_images: Vec<ImageAttachment>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
) {
    let effective_prompt = app
        .state
        .steering_prompt_prefix()
        .map(|prefix| format!("{prefix}\nUser request:\n{prompt}"))
        .unwrap_or_else(|| prompt.to_string());

    let steering_count = app.state.steering_count();
    app.state.clear_input();
    app.state.clear_steering();
    crate::tui::app::worker_bridge::handle_processing_started(app, worker_bridge).await;
    app.state.begin_request_timing();
    app.state.main_inflight_prompt = Some(prompt.to_string());
    app.state.status = if steering_count > 0 {
        format!("Submitting prompt with {steering_count} steering item(s)…")
    } else {
        "Submitting prompt…".to_string()
    };
    app.state.scroll_to_bottom();

    if let Some(reg) = registry {
        spawn_provider_task(
            app, cwd, session, reg, &effective_prompt,
            pending_images, event_tx, result_tx,
        )
        .await;
    } else {
        no_provider_error(app, worker_bridge).await;
    }
}
