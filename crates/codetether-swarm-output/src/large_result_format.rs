//! Formatting for successful RLM tool-output summaries.

use super::large_result::SIMPLE_TRUNCATE_CHARS;
use codetether_swarm_core::context::truncate_single_result;

pub(super) fn success(content: &str, tool: &str, answer: &str) -> String {
    let answer = truncate_single_result(answer, SIMPLE_TRUNCATE_CHARS * 2);
    tracing::info!(target: "codetether_agent::swarm::executor::large_result_format", tool = %tool, original_len = content.len(), summary_len = answer.len(),
        "RLM summarized large result");
    format!(
        "[RLM Summary of {tool} output ({} chars → {} chars)]\n\n{answer}",
        content.len(),
        answer.len()
    )
}
