//! Provider-safe filtering of opaque reasoning signatures during failover.

use crate::provider::{ContentPart, Message};

pub(super) fn sanitize(messages: &mut [Message], provider: &str) {
    if matches!(provider, "openai-codex" | "codex" | "chatgpt") {
        return;
    }
    for message in messages {
        message.content.retain(|part| {
            !matches!(part, ContentPart::Thinking { signature: Some(value), .. }
                if crate::provider::codex_reasoning::is_signature(value))
        });
    }
}

#[cfg(test)]
#[path = "reasoning_context_tests.rs"]
mod tests;
