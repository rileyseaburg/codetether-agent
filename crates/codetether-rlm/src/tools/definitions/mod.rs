//! Ordered tool schemas for in-memory RLM operations.
use codetether_provider_core::ToolDefinition;
mod ast_query;
mod count;
mod final_answer;
mod grep;
mod head;
mod llm_query;
mod slice;
mod tail;
/// Return all eight RLM tool definitions in their established order.
///
/// # Returns
/// Schemas for context inspection, sub-queries, and final answers.
/// # Examples
/// ```
/// assert_eq!(codetether_rlm::tools::rlm_tool_definitions().len(), 8);
/// ```
pub fn rlm_tool_definitions() -> Vec<ToolDefinition> {
    vec![
        head::definition(),
        tail::definition(),
        grep::definition(),
        count::definition(),
        slice::definition(),
        llm_query::definition(),
        final_answer::definition(),
        ast_query::definition(),
    ]
}
