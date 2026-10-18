//! Prompt preparation and provider dispatch.
//!
//! Handles steering prefix assembly, UI state updates, and
//! delegation to [`super::chat_spawn::spawn_provider_task`].

use std::{path::Path, sync::Arc};

use crate::provider::ProviderRegistry;
use crate::session::ImageAttachment;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;
use crate::tui::worker_bridge::TuiWorkerBridge;

mod host;

/// Prepare the effective prompt and dispatch to provider.
///
/// Applies steering prefix, updates UI state, and either
/// spawns the provider task or shows an error.
pub(crate) async fn dispatch_prompt(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    prompt: &str,
    pending_images: Vec<ImageAttachment>,
    runtime: &TuiSessionHandle,
) {
    codetether_chat_submit::dispatch::dispatch_prompt(
        &mut host::Native {
            app,
            cwd,
            slot,
            worker_bridge,
            runtime,
        },
        registry.as_ref(),
        prompt,
        pending_images,
    )
    .await;
}
