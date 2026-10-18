//! Provider completion-limit projection.
use super::types;
use codetether_provider_core::ModelInfo;

pub(super) fn top_provider(model: &ModelInfo) -> types::TopProvider {
    types::TopProvider {
        context_length: model.context_window,
        max_completion_tokens: model.max_output_tokens,
        is_moderated: false,
    }
}
