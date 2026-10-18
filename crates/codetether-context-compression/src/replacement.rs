//! Replacement-text construction for oversized last messages.

use std::sync::Arc;

use codetether_rlm::RlmChunker;

use super::context::CompressContext;

/// Character budget retained verbatim from the original message as a
/// prefix, after RLM compression, so the model can see the literal
/// opening of the request even when the body was summarised.
const OVERSIZED_LAST_MESSAGE_PREFIX_CHARS: usize = 500;

/// Build the compressed replacement text for an oversized last message:
/// a cached background-RLM summary when available, otherwise a
/// deterministic [`RlmChunker::compress`] projection.
pub(super) fn build_replacement(
    original: &str,
    ctx: &CompressContext,
    provider: Arc<dyn codetether_provider_core::Provider>,
    model: &str,
    threshold: usize,
    resolver: codetether_rlm_background::Resolver,
) -> String {
    let msg_tokens = RlmChunker::estimate_tokens(original);
    let cached = codetether_rlm_background::context_summary(
        original,
        "oversized_last_message",
        &ctx.session_id,
        model,
        provider,
        &ctx.rlm_config,
        resolver,
    );
    cached
        .map(|summary| {
            codetether_context_policy::compression::summary::wrap_cached(
                summary,
                original,
                msg_tokens,
                OVERSIZED_LAST_MESSAGE_PREFIX_CHARS,
            )
        })
        .unwrap_or_else(|| RlmChunker::compress(original, threshold, None))
}
