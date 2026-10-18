//! Estimate one content part, including opaque reasoning signatures.
use super::thinking;
use codetether_provider_core::ContentPart;
use codetether_rlm::RlmChunker;

/// Estimate the token cost of one provider content part.
/// # Arguments
/// * `part` — Content whose text, signature or attachment overhead is counted.
/// # Returns
/// The existing heuristic estimate for this content part.
/// # Examples
/// ```
/// use codetether_provider_core::ContentPart;
/// let part = ContentPart::Text { text: "A sufficiently long message".into() };
/// assert!(codetether_request_tokens::estimate_tokens_for_part(&part) > 0);
/// ```
pub fn estimate_tokens_for_part(part: &ContentPart) -> usize {
    match part {
        ContentPart::Text { text } => RlmChunker::estimate_tokens(text),
        ContentPart::ToolResult { content, .. } => RlmChunker::estimate_tokens(content),
        ContentPart::ToolCall {
            id,
            name,
            arguments,
            thought_signature,
            ..
        } => {
            let mut s = String::new();
            s.push_str(id);
            s.push(' ');
            s.push_str(name);
            s.push(' ');
            s.push_str(arguments);
            if let Some(sig) = thought_signature {
                s.push(' ');
                s.push_str(sig);
            }
            RlmChunker::estimate_tokens(&s)
        }
        ContentPart::Thinking { text, signature } => thinking::estimate(text, signature),
        ContentPart::Image { .. } => 2000,
        ContentPart::File { path, mime_type } => {
            let mut s = String::new();
            s.push_str(path);
            if let Some(mt) = mime_type {
                s.push(' ');
                s.push_str(mt);
            }
            RlmChunker::estimate_tokens(&s)
        }
    }
}
