//! Background summarisation for Reset-policy dropped prefixes.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::provider::Message;
use crate::rlm::RlmChunker;
use crate::session::SessionEvent;
use crate::session::helper::error::messages_to_rlm_context;

/// Summarise the prefix dropped by
/// [`derive_reset`](super::reset::derive_reset).
#[allow(clippy::too_many_arguments)]
pub(super) async fn summarise_prefix_for_reset(
    prefix: &[Message],
    session_id: &str,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    rlm_config: &crate::rlm::RlmConfig,
    _subcall_provider: Option<Arc<dyn crate::provider::Provider>>,
    _subcall_model: Option<String>,
    _event_tx: Option<&mpsc::Sender<SessionEvent>>,
) -> Result<String> {
    let context = messages_to_rlm_context(prefix);
    let cached = crate::session::helper::rlm_background::context_summary(
        &context,
        "context_reset",
        session_id,
        model,
        provider,
        rlm_config,
    );
    Ok(cached.unwrap_or_else(|| RlmChunker::compress(&context, 512, None)))
}
