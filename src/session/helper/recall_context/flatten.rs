//! Budget-aware session flattening for recall.

use crate::provider::Message;
use crate::rlm::RlmChunker;
use super::render::render_part;

/// Maximum estimated tokens per session for recall.
/// Stays below RLM's 50K compression threshold.
const RECALL_TOKEN_BUDGET: usize = 30_000;

/// Convert messages to a flat recall string with default budget.
pub fn flatten_messages(messages: &[Message]) -> (String, bool) {
    flatten_messages_with_budget(messages, RECALL_TOKEN_BUDGET)
}

/// Budget-aware flattening accepting a custom token limit.
pub fn flatten_messages_with_budget(
    messages: &[Message],
    budget: usize,
) -> (String, bool) {
    let mut out = String::with_capacity(budget * 4);
    let mut truncated = false;

    for (idx, m) in messages.iter().enumerate() {
        let role = super::super::text::role_label(m.role);
        let header = format!("[{idx} {role}]\n");

        if RlmChunker::estimate_tokens(&out) + RlmChunker::estimate_tokens(&header) > budget {
            truncated = true;
            break;
        }
        out.push_str(&header);

        for part in &m.content {
            let segment = render_part(part);
            let seg_tokens = RlmChunker::estimate_tokens(&segment);
            let current = RlmChunker::estimate_tokens(&out);

            if current + seg_tokens > budget {
                out.push_str("[...truncated]\n");
                truncated = true;
                break;
            }
            out.push_str(&segment);
        }

        out.push_str("\n---\n\n");

        if RlmChunker::estimate_tokens(&out) >= budget {
            truncated = true;
            break;
        }
    }

    (out, truncated)
}
