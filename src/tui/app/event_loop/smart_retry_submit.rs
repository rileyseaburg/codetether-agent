//! Submit smart-switch retries through the session runtime.

use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::tui::app::session_runtime::{PromptRequest, SessionSlot, TuiSessionHandle};

pub(super) async fn submit(
    slot: &mut SessionSlot,
    prompt: &str,
    runtime: &TuiSessionHandle,
    registry: &Arc<ProviderRegistry>,
) {
    let Some(session) = slot.take_for_prompt() else {
        return;
    };
    let original_dir = session.metadata.directory.clone();
    let request = PromptRequest::new(
        session,
        prompt.to_string(),
        Vec::new(),
        Arc::clone(registry),
        original_dir,
        None,
    );
    if let Err(request) = runtime.submit(request).await {
        slot.restore(request.session);
    }
}
