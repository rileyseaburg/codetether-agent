//! Non-blocking summary production helpers.

use std::sync::Arc;

use codetether_provider_core::Provider;
use codetether_rlm::{RlmChunker, RlmConfig};

pub(super) fn summary_or_fallback(
    input: &str,
    target_tokens: usize,
    provider: Arc<dyn Provider>,
    model: &str,
    rlm_config: &RlmConfig,
    session_id: &str,
    resolver: codetether_rlm_background::Resolver,
) -> String {
    let cached = codetether_rlm_background::context_summary(
        input,
        "summary_index",
        session_id,
        model,
        provider,
        rlm_config,
        resolver,
    );
    cached
        .map(|s| codetether_summary_quality::clamp_tokens(&s, target_tokens))
        .unwrap_or_else(|| RlmChunker::compress(input, target_tokens, None))
}
