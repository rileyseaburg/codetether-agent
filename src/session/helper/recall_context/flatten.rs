//! Budget-aware session flattening for recall.

use crate::provider::Message;
use crate::rlm::RlmChunker;
use super::render::render_part;

/// Maximum estimated tokens per session for recall.
const RECALL_TOKEN_BUDGET: usize = 30_000;

/// Convert messages to a flat recall string with default budget.
pub fn flatten_messages(messages: &[Message]) -> (String, bool) {
    flatten_messages_with_budget(messages, RECALL_TOKEN_BUDGET)
}

/// Budget-aware flattening with a running token count (O(N), not O(N²)).
pub fn flatten_messages_with_budget(
    messages: &[Message],
    budget: usize,
) -> (String, bool) {
    let mut out = String::with_capacity(budget * 4);
    let mut tokens = 0usize;
    let mut truncated = false;
    let sep = "\n---\n\n";
    let sep_tok = RlmChunker::estimate_tokens(sep);
    let marker = "[...truncated]\n";
    let marker_tok = RlmChunker::estimate_tokens(marker);

    for (idx, m) in messages.iter().enumerate() {
        let role = super::super::text::role_label(m.role);
        let hdr = format!("[{idx} {role}]\n");
        let hdr_tok = RlmChunker::estimate_tokens(&hdr);

        if tokens + hdr_tok > budget { truncated = true; break; }
        out.push_str(&hdr);
        tokens += hdr_tok;

        for part in &m.content {
            let seg = render_part(part);
            let seg_tok = RlmChunker::estimate_tokens(&seg);
            if tokens + seg_tok > budget {
                if tokens + marker_tok <= budget { out.push_str(marker); tokens += marker_tok; }
                truncated = true;
                break;
            }
            out.push_str(&seg);
            tokens += seg_tok;
        }

        if tokens + sep_tok > budget { truncated = true; break; }
        out.push_str(sep);
        tokens += sep_tok;
    }

    (out, truncated)
}
