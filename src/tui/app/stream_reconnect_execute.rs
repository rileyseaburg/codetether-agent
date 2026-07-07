//! Execute a pending stream-reconnect retry.
//!
//! Applies exponential backoff (10s × 2^attempt) before re-submitting so the
//! provider has time to recover after SRP exhausted its 3-stream-restart budget.

use std::sync::Arc;
use std::time::Duration;

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{PromptRequest, SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;

use super::stream_reconnect::{push_reconnect_banner, reset_on_success};

/// Re-submit the pending prompt after a stream disconnect.
///
/// No-ops when `pending_stream_reconnect` is `None` or the session is busy.
pub(crate) async fn execute(
    app: &mut App,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    runtime: &TuiSessionHandle,
) {
    if app.state.processing {
        return;
    }
    let Some(prompt) = app.state.pending_stream_reconnect.take() else {
        return;
    };
    let Some(registry) = registry.as_ref() else {
        app.state.pending_stream_reconnect = Some(prompt);
        return;
    };
    let Some(session) = slot.take_for_prompt() else {
        app.state.pending_stream_reconnect = Some(prompt);
        return;
    };
    push_reconnect_banner(app);
    app.state.processing = true;
    // Backoff: 10s / 20s / 40s for attempts 1 / 2 / 3.
    let secs = 10 * 2u64.saturating_pow(app.state.stream_reconnect_count.saturating_sub(1));
    tokio::time::sleep(Duration::from_secs(secs)).await;
    app.state.begin_request_timing();
    app.state.main_inflight_prompt = Some(prompt.clone());
    app.state.main_last_event_at = Some(std::time::Instant::now());
    let dir = session.metadata.directory.clone();
    let request = PromptRequest::new(session, prompt, Vec::new(), Arc::clone(registry), dir, None);
    if let Err(req) = runtime.submit(request).await {
        slot.restore(req.session);
        app.state.processing = false;
    }
}

/// Called when a turn finishes successfully — resets the reconnect budget.
pub(crate) fn on_success(app: &mut App) {
    reset_on_success(app);
}
