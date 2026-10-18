//! Append bounded content parts while preserving the original truncation marker.
use codetether_provider_core::ContentPart;
use codetether_recall_index::part::render_part;
use codetether_rlm::RlmChunker;
pub(super) fn append(
    out: &mut String,
    tokens: &mut usize,
    parts: &[ContentPart],
    budget: usize,
    marker: &str,
    marker_tok: usize,
) -> bool {
    for part in parts {
        let seg = render_part(part);
        let seg_tok = RlmChunker::estimate_tokens(&seg);
        if *tokens + seg_tok > budget {
            if *tokens + marker_tok <= budget {
                out.push_str(marker);
                *tokens += marker_tok;
            }
            return true;
        }
        out.push_str(&seg);
        *tokens += seg_tok;
    }
    false
}
