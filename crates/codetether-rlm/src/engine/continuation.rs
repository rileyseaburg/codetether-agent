//! Chronological evidence for continuation: retrieval relevance cannot drop turns.
use super::evidence::Evidence;
use crate::RlmChunker;

pub(super) fn is_continuation(tool: &str) -> bool {
    matches!(
        tool,
        "session_context" | "context_reset" | "summary_index" | "proactive_session_context"
    )
}

pub(super) fn collect(content: &str) -> Evidence {
    let mut starts = vec![0];
    let mut offset = 0;
    for line in content.split_inclusive('\n') {
        if offset > 0 && is_turn(line.trim_end()) {
            starts.push(offset);
        }
        offset += line.len();
    }
    starts.push(content.len());
    let sections: Vec<_> = starts.windows(2).map(|w| &content[w[0]..w[1]]).collect();
    let text = crate::text_budget::pack_sections(&sections, 64_000);
    Evidence {
        text,
        input_tokens: RlmChunker::estimate_tokens(content),
        records: Vec::new(),
    }
}

fn is_turn(line: &str) -> bool {
    let Some(inner) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) else {
        return false;
    };
    let Some((index, role)) = inner.split_once(' ') else {
        return false;
    };
    index.parse::<usize>().is_ok()
        && matches!(role, "System" | "Developer" | "User" | "Assistant" | "Tool")
}
