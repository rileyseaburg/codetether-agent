//! Non-blocking summary production helpers.

use std::sync::Arc;

use crate::provider::Provider;
use crate::rlm::{RlmChunker, RlmConfig};

pub(super) fn summary_or_fallback(
    input: &str,
    target_tokens: usize,
    provider: Arc<dyn Provider>,
    model: &str,
    rlm_config: &RlmConfig,
    session_id: &str,
) -> String {
    let cached = crate::session::helper::rlm_background::context_summary(
        input,
        "summary_index",
        session_id,
        model,
        provider,
        rlm_config,
    );
    cached
        .map(|s| super::summary_text::clamp_tokens(&s, target_tokens))
        .unwrap_or_else(|| RlmChunker::compress(input, target_tokens, None))
}
