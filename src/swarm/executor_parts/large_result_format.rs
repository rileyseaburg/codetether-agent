//! Formatting for successful RLM tool-output summaries.

use super::large_result::SIMPLE_TRUNCATE_CHARS;
use crate::swarm::token_truncate::truncate_single_result;

pub(super) fn success(content: &str, tool: &str, answer: &str) -> String {
    let answer = truncate_single_result(answer, SIMPLE_TRUNCATE_CHARS * 2);
    tracing::info!(tool = %tool, original_len = content.len(), summary_len = answer.len(),
        "RLM summarized large result");
    format!(
        "[RLM Summary of {tool} output ({} chars → {} chars)]\n\n{answer}",
        content.len(),
        answer.len()
    )
}
