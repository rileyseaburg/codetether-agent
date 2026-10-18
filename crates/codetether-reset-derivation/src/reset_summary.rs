//! Background summarisation for Reset-policy dropped prefixes.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use codetether_context_policy::transcript::messages_to_rlm_context;
use codetether_provider_core::Message;
use codetether_rlm::RlmChunker;
use codetether_session_events::SessionEvent;

/// Summarise the prefix dropped by
/// [`derive_reset`](super::reset::derive_reset).
#[allow(clippy::too_many_arguments)]
pub(super) async fn summarise_prefix_for_reset<S>(
    prefix: &[Message],
    session_id: &str,
    provider: Arc<dyn codetether_provider_core::Provider>,
    model: &str,
    rlm_config: &codetether_rlm::RlmConfig,
    _subcall_provider: Option<Arc<dyn codetether_provider_core::Provider>>,
    _subcall_model: Option<String>,
    _event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
    resolver: codetether_rlm_background::Resolver,
) -> Result<String> {
    let context = messages_to_rlm_context(prefix);
    let cached = codetether_rlm_background::context_summary(
        &context,
        "context_reset",
        session_id,
        model,
        provider,
        rlm_config,
        resolver,
    );
    Ok(cached.unwrap_or_else(|| RlmChunker::compress(&context, 512, None)))
}
