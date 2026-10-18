//! Execute a pending stream-reconnect retry.
//!
//! Applies exponential backoff (10s × 2^attempt) before re-submitting so the
//! provider has time to recover after SRP exhausted its 3-stream-restart budget.

use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;

use super::stream_reconnect::reset_on_success;
mod host;

/// Re-submit the pending prompt after a stream disconnect.
///
/// No-ops when `pending_stream_reconnect` is `None` or the session is busy.
pub(crate) async fn execute(
    app: &mut App,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    runtime: &TuiSessionHandle,
) {
    codetether_stream_recovery::retry::execute(
        &mut host::Native { app, slot, runtime },
        registry.as_ref(),
    )
    .await;
}

/// Called when a turn finishes successfully — resets the reconnect budget.
pub(crate) fn on_success(app: &mut App) {
    reset_on_success(app);
}
