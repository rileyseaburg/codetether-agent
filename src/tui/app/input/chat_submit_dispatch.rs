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
pub(crate) async fn dispatch_prompt(
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
    app.state.clear_input();
    crate::tui::app::worker_bridge::handle_processing_started(app, worker_bridge).await;
    app.state.begin_request_timing();
    app.state.main_inflight_prompt = Some(prompt.to_string());
    app.state.status = "Submitting prompt…".to_string();
    app.state.scroll_to_bottom();

    if let Some(reg) = registry {
        spawn_provider_task(
            app,
            cwd,
            session,
            reg,
            prompt,
            pending_images,
            event_tx,
            result_tx,
        )
        .await;
    } else {
        no_provider_error(app, worker_bridge).await;
    }
}
