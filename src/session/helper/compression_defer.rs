//! Fast fallback for foreground context compaction.

use std::sync::Arc;

use crate::provider::Provider;
use crate::rlm::{RlmChunker, RlmConfig};

pub(super) fn context_summary(
    context: &str,
    ctx_window: usize,
    session_id: &str,
    reason: &str,
    model: &str,
    provider: Arc<dyn Provider>,
    config: &RlmConfig,
) -> Option<String> {
    let tokens = RlmChunker::estimate_tokens(context);
    if tokens == 0 {
        return None;
    }
    let cached = super::rlm_background::context_summary(
        context, reason, session_id, model, provider, config,
    );
    tracing::info!(tokens, "RLM context compaction deferred to background");
    Some(cached.unwrap_or_else(|| {
        RlmChunker::compress(context, (ctx_window as f64 * 0.25) as usize, None)
    }))
}
