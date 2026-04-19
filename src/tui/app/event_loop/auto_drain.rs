//! Auto-drain queued user input after a turn completes.
//!
//! When the user types a message while the agent is still responding,
//! the input is parked in [`App::queued_steering`]. Previously this only
//! got applied on the user's *next* manual submit, so a queued message
//! would "just sit there" after the current turn finished.
//!
//! This module wires up the obvious fix: as soon as processing goes
//! idle, if there is queued input, treat it as a fresh user prompt and
//! kick off a new turn automatically. From the user's point of view it
//! behaves like "type anytime, your message will be sent when the agent
//! is free".

use std::path::Path;
use std::sync::Arc;

use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{Session, SessionEvent};
use crate::tui::app::input::{dispatch_prompt, push_user_messages};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

/// If processing is idle and [`App::queued_steering`] is non-empty,
/// join the queued items into a single user prompt and submit it.
///
/// Called by the event loop after [`super::background::apply_single_result`]
/// has settled (or the user cancelled), so that a message typed
/// mid-stream isn't stranded.
pub(super) async fn auto_drain_queued_input(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
) {
    if app.state.processing {
        return;
    }
    if app.state.queued_steering.is_empty() {
        return;
    }

    // Drain the queue *before* dispatch: dispatch_prompt expects
    // `queued_steering` to represent prefix guidance for the *current*
    // prompt, not the prompt itself. Joining them with blank lines
    // matches how a user would type multiple paragraphs.
    let prompt = std::mem::take(&mut app.state.queued_steering).join("\n\n");
    let trimmed = prompt.trim().to_string();
    // Also take any images the user attached while queueing — queue_steering
    // leaves them on `pending_images` so they travel with the auto-submit.
    let pending_images = std::mem::take(&mut app.state.pending_images);
    if trimmed.is_empty() && pending_images.is_empty() {
        return;
    }

    // Mirror the manual-submit path so the chat view shows the queued
    // text as a normal user message, then dispatch as a fresh turn.
    push_user_messages(app, &trimmed, &pending_images);
    app.state.status = "Submitting queued message…".to_string();

    dispatch_prompt(
        app,
        cwd,
        session,
        registry,
        worker_bridge,
        &trimmed,
        pending_images,
        event_tx,
        result_tx,
    )
    .await;
}
