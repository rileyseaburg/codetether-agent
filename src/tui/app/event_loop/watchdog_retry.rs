//! Submit watchdog retries after cancellation restores the session.
//!
//! After the watchdog cancels a stalled request, this module resubmits the
//! same prompt. If the retry budget ([`WATCHDOG_MAX_RESTARTS`]) is exhausted,
//! it delegates to [`give_up`] to surface a permanent error.

#[path = "watchdog_give_up.rs"]
mod give_up;

use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{PromptRequest, SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::constants::WATCHDOG_MAX_RESTARTS;

pub(super) async fn execute(
    app: &mut App,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    runtime: &TuiSessionHandle,
) {
    if app.state.watchdog_notification.is_none() || app.state.processing {
        return;
    }
    if app.state.main_watchdog_restart_count >= WATCHDOG_MAX_RESTARTS {
        return give_up::surface(app);
    }
    let Some(registry) = registry.as_ref() else {
        return;
    };
    let prompt = app
        .state
        .main_watchdog_root_prompt
        .clone()
        .or_else(|| app.state.main_inflight_prompt.clone());
    let Some(prompt) = prompt else { return };
    let Some(session) = slot.take_for_prompt() else {
        return;
    };
    app.state.main_inflight_prompt = Some(prompt.clone());
    app.state.processing = true;
    app.state.begin_request_timing();
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
    if let Err(request) = runtime.submit(request).await {
        slot.restore(request.session);
    }
}
