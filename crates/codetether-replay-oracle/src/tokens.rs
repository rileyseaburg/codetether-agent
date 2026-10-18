//! Reuse relevance extraction for standalone text file references.
use codetether_context_policy::relevance::extract;
use codetether_provider_core::{ContentPart, Message};

/// Re-use the relevance extractor on a synthetic single-text message
/// to get the file tokens. Duplicated logic is cheaper than exposing
/// the internal token-splitter.
pub(crate) fn extract_text_file_tokens(text: &str) -> Vec<String> {
    use codetether_provider_core::Role;
    let synthetic = Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: text.to_string(),
        }],
    };
    extract(&synthetic).files
}
