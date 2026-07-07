//! Execute a pending stream-reconnect retry.
//!
//! Called from the select-loop notice branch after `background_notice_failed`
//! has restored the session. If a reconnect was scheduled, this re-submits
//! the original prompt on the same model without switching providers.

use std::sync::Arc;

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
    app.state.begin_request_timing();
    app.state.main_inflight_prompt = Some(prompt.clone());
    app.state.main_last_event_at = Some(std::time::Instant::now());
    let original_dir = session.metadata.directory.clone();
    let request = PromptRequest::new(
        session,
        prompt,
        Vec::new(),
        Arc::clone(registry),
        original_dir,
        None,
    );
    if let Err(req) = runtime.submit(request).await {
        slot.restore(req.session);
        app.state.processing = false;
    }
}

/// Called when a turn finishes successfully — resets the reconnect budget.
pub(crate) fn on_success(app: &mut App) {
    reset_on_success(app);
}
