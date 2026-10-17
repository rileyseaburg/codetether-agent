//! Caller-supplied controls used by the inference serialization regressions.

use codetether_provider_core::CompletionRequest;

pub(super) fn request(model: &str) -> CompletionRequest {
    CompletionRequest {
        model: model.into(),
        messages: vec![],
        tools: vec![],
        temperature: Some(0.7),
        top_p: None,
        max_tokens: Some(128),
        stop: vec![],
    }
}
