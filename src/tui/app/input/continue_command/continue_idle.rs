//! Resume an idle session for `/continue`.
//!
//! When no turn is in flight, `/continue` simply resubmits the resolved prompt
//! as a fresh turn: push it as a user message, then dispatch through the same
//! path a normal Enter would take.

use std::path::Path;
use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

use super::super::chat_helpers::push_user_messages;
use super::super::chat_submit_dispatch::dispatch_prompt;

/// Resubmit `prompt` as a new turn on an idle session.
#[allow(clippy::too_many_arguments)]
pub(super) async fn resume(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    prompt: String,
    runtime: &TuiSessionHandle,
) {
    app.state.status = "Continuing — resubmitting prompt…".to_string();
    push_user_messages(app, &prompt, &[]);
    dispatch_prompt(
        app,
        cwd,
        slot,
        registry,
        worker_bridge,
        &prompt,
        Vec::new(),
        runtime,
    )
    .await;
}
