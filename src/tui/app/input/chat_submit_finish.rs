//! Final stage of chat submission: take pending attachments, expand paste
//! placeholders, push the user message, and dispatch to the provider.

use std::{path::Path, sync::Arc};

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

#[path = "chat_submit_goal.rs"]
mod goal;
mod host;

/// Dispatch `prompt` plus any pending images/text-paste sidecars.
pub(super) async fn finish_submit(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    runtime: &TuiSessionHandle,
    prompt: &str,
) {
    codetether_chat_submit::finish::finish(
        &mut host::Native {
            app,
            cwd,
            slot,
            registry,
            worker_bridge,
            runtime,
        },
        prompt,
    )
    .await;
}
