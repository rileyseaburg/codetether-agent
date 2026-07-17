//! Token accounting for visible and opaque reasoning content.

use crate::rlm::RlmChunker;

pub(super) fn estimate(text: &str, signature: &Option<String>) -> usize {
    RlmChunker::estimate_tokens(text)
        + signature
            .as_deref()
            .map(RlmChunker::estimate_tokens)
            .unwrap_or(0)
}

#[cfg(test)]
#[path = "thinking_tests.rs"]
mod tests;
