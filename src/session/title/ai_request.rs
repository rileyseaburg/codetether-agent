//! Builds the `CompletionRequest` used for AI title synthesis.

use crate::provider::{CompletionRequest, ContentPart, Message, Role};

use super::super::helper::text::truncate_with_ellipsis;

/// System prompt instructing the model to emit only a terse title.
const TITLE_SYSTEM: &str = "You name developer chat sessions. Reply with ONLY a concise \
title of at most 6 words. No quotes, no punctuation at the end, no preamble.";

/// Build the title-synthesis request for `model`, seeded by user text.
pub(super) fn title_request(model: String, seed: &str) -> CompletionRequest {
    CompletionRequest {
        model,
        messages: vec![
            text_message(Role::System, TITLE_SYSTEM.to_string()),
            text_message(Role::User, truncate_with_ellipsis(seed, 800)),
        ],
        tools: Vec::new(),
        temperature: Some(0.2),
        top_p: Some(0.9),
        max_tokens: Some(24),
        stop: Vec::new(),
    }
}

fn text_message(role: Role, text: String) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text }],
    }
}
