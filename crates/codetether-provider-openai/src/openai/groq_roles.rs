//! Groq Qwen templates accept system guidance but reject the developer role.
use codetether_provider_core::{CompletionRequest, Role};

pub(super) fn prepare(request: &mut CompletionRequest) {
    let model = request
        .model
        .strip_prefix("groq/")
        .unwrap_or(&request.model);
    if !model.starts_with("qwen/") {
        return;
    }
    // Adapt only the transport-owned request. Retain ordering, content, and tool
    // associations; never demote internal guidance to user text. GPT-OSS keeps
    // its native developer role, as do Qwen requests through other providers.
    for message in &mut request.messages {
        if message.role == Role::Developer {
            message.role = Role::System;
        }
    }
}
