//! Automatic continuation of an active goal when a session becomes idle.

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;
use std::{path::Path, sync::Arc};

pub(crate) async fn resume(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
) -> bool {
    if app.state.processing || registry.is_none() {
        return false;
    }
    let Some(prompt) = slot
        .borrow()
        .and_then(|session| crate::session::tasks::runtime::resume_prompt(&session.id))
    else {
        return false;
    };
    app.state.status = "Resuming active goal…".to_string();
    super::super::chat_helpers::push_user_messages(app, &prompt, &[]);
    super::super::chat_submit_dispatch::dispatch_prompt(
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
    true
}
